// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

use std::rc::Rc;

use crate::compute_coords_from_placement;
use crate::traits::Element;
use crate::types::{
    ComputePositionConfig, ComputePositionReturn, Elements, MiddlewareData, MiddlewareState,
};

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

    // TODO(Shaohua): valid middlewares
    let rtl = platform.is_rtl(floating_element);
    let rects = platform.element_rects(reference_element, floating_element, strategy);
    let mut coords = compute_coords_from_placement(&rects, placement, rtl);
    let stateful_placement = placement;
    let middleware_data = MiddlewareData::default();
    let elements = Elements {
        floating: floating_element.clone(),
        reference: reference_element.clone(),
    };

    for middleware in middlewares.iter() {
        let mut state = MiddlewareState {
            coords: coords.clone(),
            initial_placement: placement,
            placement: stateful_placement,
            strategy,
            middleware_data: middleware_data.clone(),
            rects: rects.clone(),
            platform: platform.clone(),
            elements: elements.clone(),
        };
        let middleware_return = middleware.run(&mut state);
        if let Some(x) = middleware_return.coords.x {
            coords.x = x;
        };
        if let Some(y) = middleware_return.coords.y {
            coords.y = y;
        }

        // TODO(Shaohua): Add more stmt
    }

    ComputePositionReturn {
        coords,
        strategy,
        placement: stateful_placement,
        middleware_data,
    }
}
