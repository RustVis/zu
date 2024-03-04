// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

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
