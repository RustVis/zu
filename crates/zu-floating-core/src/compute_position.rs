// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::compute_coords::compute_coords_from_placement;
use crate::middleware::{
    Middleware, MiddlewareData, MiddlewareResetRects, MiddlewareReturn, MiddlewareState,
};
use crate::platform::{Element, Elements, Platform};
use crate::types::{Coords, Placement, Strategy};

const MAX_RESET_COUNT: usize = 50;

pub struct ComputePositionConfig {
    pub platform: Rc<dyn Platform>,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middlewares: Vec<Box<dyn Middleware>>,
}

impl fmt::Debug for ComputePositionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComputePositionConfig")
            .field("platform", &self.platform)
            .field("placement", &self.placement)
            .field("strategy", &self.strategy)
            .field("middlewares", &self.middlewares)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct ComputePositionReturn {
    pub coords: Coords,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
}

pub type ComputePosition = fn(
    reference_element: &Rc<dyn Element>,
    floating_element: &Rc<dyn Element>,
    config: &ComputePositionConfig,
) -> ComputePositionReturn;

/// Computes the `x` and `y` coordinates that will place the floating element
/// next to a given reference element.
///
/// This export does not have any `platform` interface logic.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn compute_position(
    reference_element: &Rc<dyn Element>,
    floating_element: &Rc<dyn Element>,
    config: &ComputePositionConfig,
) -> ComputePositionReturn {
    let platform = &config.platform;
    let placement = config.placement;
    let strategy = config.strategy;
    let middlewares = &config.middlewares;

    let rtl = platform.is_rtl(floating_element);
    let mut element_rects = platform.element_rects(reference_element, floating_element, strategy);
    let mut coords = compute_coords_from_placement(&element_rects, placement, rtl);
    let mut stateful_placement = placement;
    let mut middleware_data = MiddlewareData::default();
    let elements = &Elements {
        floating: floating_element.clone(),
        reference: reference_element.clone(),
    };
    let mut reset_count = 0;
    let mut index = 0;
    while index < middlewares.len() {
        let middleware = middlewares.get(index).unwrap();
        let state = MiddlewareState {
            coords: &coords,
            initial_placement: placement,
            placement: stateful_placement,
            strategy,
            middleware_data: &middleware_data,
            rects: &element_rects,
            platform,
            elements,
        };

        let mut middleware_return: MiddlewareReturn = middleware.run(&state);

        // Merge new coordinates.
        if let Some(x) = middleware_return.coords.x {
            coords.x = x;
        };
        if let Some(y) = middleware_return.coords.y {
            coords.y = y;
        }

        // Merge middleware data.
        middleware_data.append(&mut middleware_return.data);

        // Check reset state
        if middleware_return.reset.is_on && reset_count <= MAX_RESET_COUNT {
            reset_count += 1;
            index = 0;

            if let Some(placement) = middleware_return.reset.placement {
                stateful_placement = placement;
            }

            element_rects = match middleware_return.reset.rects {
                MiddlewareResetRects::Nil => element_rects,
                MiddlewareResetRects::FromPlatform => {
                    platform.element_rects(reference_element, floating_element, strategy)
                }
                MiddlewareResetRects::Value(new_rects) => new_rects,
            };

            coords = compute_coords_from_placement(&element_rects, stateful_placement, rtl);
        } else {
            index += 1;
        }
    }

    ComputePositionReturn {
        coords,
        strategy,
        placement: stateful_placement,
        middleware_data,
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::compute_position::{compute_position, ComputePositionConfig};
    use crate::middleware::{Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState};
    use crate::types::{PartialCoords, Placement, Strategy};
    use crate::vanilla::VanillaPlatform;

    #[test]
    fn test_returned_data() {
        #[derive(Debug)]
        struct CustomMiddlewareData {
            pub property: bool,
        }

        #[derive(Debug)]
        struct CustomMiddleware {}

        impl CustomMiddleware {
            pub const NAME: &'static str = "custom";

            #[must_use]
            pub fn new() -> Self {
                Self {}
            }
        }

        impl Middleware for CustomMiddleware {
            fn name(&self) -> &str {
                Self::NAME
            }

            fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
                let prop = Box::new(CustomMiddlewareData { property: true });
                let data = MiddlewareData::with_value(Self::NAME, prop);
                MiddlewareReturn {
                    data,
                    ..Default::default()
                }
            }
        }

        let platform = Rc::new(VanillaPlatform::new());
        let reference_element = platform.reference_element();
        let floating_element = platform.floating_element();
        let config = ComputePositionConfig {
            platform: platform.clone(),
            placement: Placement::Top,
            strategy: Strategy::default(),
            middlewares: vec![Box::new(CustomMiddleware::new())],
        };
        let returned_data = compute_position(reference_element, floating_element, &config);
        assert_eq!(returned_data.placement, Placement::Top);
        assert_eq!(returned_data.strategy, Strategy::Absolute);
        assert_eq!(returned_data.coords.x, 25.0);
        assert_eq!(returned_data.coords.y, 25.0);
        let custom_data = returned_data.middleware_data.get(CustomMiddleware::NAME);
        assert!(custom_data.is_some());
        let custom_data = custom_data.unwrap();
        if let Some(custom_data) = custom_data.downcast_ref::<CustomMiddlewareData>() {
            assert_eq!(custom_data.property, true);
        }
    }

    #[test]
    fn test_middleware() {
        #[derive(Debug)]
        struct TestMiddlewareData {
            pub hello: bool,
        }

        #[derive(Debug)]
        struct TestMiddleware {}

        impl TestMiddleware {
            pub const NAME: &'static str = "test";

            #[must_use]
            pub fn new() -> Self {
                Self {}
            }
        }

        impl Middleware for TestMiddleware {
            fn name(&self) -> &str {
                Self::NAME
            }

            fn run(&self, state: &MiddlewareState) -> MiddlewareReturn {
                let prop = Box::new(TestMiddlewareData { hello: true });
                let data = MiddlewareData::with_value(Self::NAME, prop);
                MiddlewareReturn {
                    coords: PartialCoords {
                        x: Some(state.coords.x + 1.0),
                        y: Some(state.coords.y + 1.0),
                    },
                    data,
                    ..Default::default()
                }
            }
        }

        let platform = Rc::new(VanillaPlatform::new());
        let reference_element = platform.reference_element();
        let floating_element = platform.floating_element();
        let config = ComputePositionConfig {
            platform: platform.clone(),
            placement: Placement::Top,
            strategy: Strategy::default(),
            middlewares: Vec::new(),
        };
        let returned_data = compute_position(reference_element, floating_element, &config);
        let config2 = ComputePositionConfig {
            platform: platform.clone(),
            placement: Placement::Top,
            strategy: Strategy::default(),
            middlewares: vec![Box::new(TestMiddleware::new())],
        };
        let returned_data2 = compute_position(reference_element, floating_element, &config2);
        assert_eq!(returned_data2.coords.x, returned_data.coords.x + 1.0);
        assert_eq!(returned_data2.coords.y, returned_data.coords.y + 1.0);
    }
}
