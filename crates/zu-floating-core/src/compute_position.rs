// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::compute_coords::compute_coords_from_placement;
use crate::middleware::{Middleware, MiddlewareData, MiddlewareState};
use crate::platform::{Element, Elements, Platform};
use crate::types::{Coords, Placement, Strategy};

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
    let element_rects = platform.element_rects(reference_element, floating_element, strategy);
    let coords = compute_coords_from_placement(&element_rects, placement, rtl);
    let stateful_placement = placement;
    let middleware_data = MiddlewareData::default();
    let elements = &Elements {
        floating: floating_element.clone(),
        reference: reference_element.clone(),
    };
    let mut state = MiddlewareState {
        coords,
        initial_placement: placement,
        placement: stateful_placement,
        strategy,
        middleware_data: &middleware_data,
        rects: &element_rects,
        platform,
        elements,
    };

    for middleware in middlewares {
        let middleware_return = middleware.run(&state);
        if let Some(x) = middleware_return.coords.x {
            state.coords.x = x;
        };
        if let Some(y) = middleware_return.coords.y {
            state.coords.y = y;
        }

        // TODO(Shaohua): Extend middleware_data
        // middleware_data.extend(&middleware_return.data);

        // TODO(Shaohua): Reset
    }

    ComputePositionReturn {
        coords: state.coords,
        strategy,
        placement: stateful_placement,
        middleware_data,
    }
}
