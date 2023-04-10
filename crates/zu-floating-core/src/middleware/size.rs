// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::detect_overflow::{detect_overflow, DetectOverflowOption};
use crate::traits::{Middleware, SideTrait};
use crate::types::{
    Alignment, Axis, Dimensions, MiddlewareData, MiddlewareDataKind, MiddlewareReturn,
    MiddlewareState, Rect, Side,
};
use float_cmp::ApproxEq;

pub trait SizeOptionTrait {
    fn apply(&self, state: &mut MiddlewareState, available_width: f64, available_height: f64);
}

pub struct Size {
    pub option: Box<dyn SizeOptionTrait>,
    pub detect_overflow_option: DetectOverflowOption,
}

impl Middleware for Size {
    fn kind(&self) -> MiddlewareDataKind {
        MiddlewareDataKind::Size
    }

    fn run(&self, state: &mut MiddlewareState) -> MiddlewareReturn {
        let placement = state.placement;
        let platform = &state.platform;
        let rects = &state.rects;
        let elements = &state.elements;

        let overflow = detect_overflow(state, &self.detect_overflow_option);
        let side = placement.side();
        let alignment = placement.alignment();
        let axis = placement.main_axis();
        let is_x_axis = axis == Axis::X;
        let Rect { width, height, .. } = rects.floating;

        let (width_side, height_side) = match side {
            Side::Top | Side::Bottom => {
                let element_alignment = if platform.is_rtl(&elements.floating) {
                    Alignment::Start
                } else {
                    Alignment::End
                };
                let width_side = if alignment == Some(element_alignment) {
                    Side::Left
                } else {
                    Side::Right
                };
                (width_side, side)
            }
            _ => {
                let height_side = if alignment == Some(Alignment::End) {
                    Side::Top
                } else {
                    Side::Bottom
                };
                (side, height_side)
            }
        };

        let overflow_available_width = width - overflow.side(width_side);
        let overflow_available_height = height - overflow.side(height_side);

        let mut available_width = if is_x_axis {
            (width - overflow.right - overflow.left).min(overflow_available_width)
        } else {
            overflow_available_width
        };
        let mut available_height = if is_x_axis {
            overflow_available_height
        } else {
            (height - overflow.bottom - overflow.top).min(overflow_available_height)
        };

        if state.middleware_data.shift.is_none() && alignment.is_none() {
            let x_min = overflow.left.max(0.0);
            let x_max = overflow.right.max(0.0);
            let y_min = overflow.top.max(0.0);
            let y_max = overflow.bottom.max(0.0);

            if is_x_axis {
                available_width = if x_min != 0.0 || x_max != 0.0 {
                    2.0_f64.mul_add(-(x_min + x_max), width)
                } else {
                    2.0_f64.mul_add(-overflow.left.max(overflow.right), width)
                };
            } else {
                available_height = if y_min != 0.0 || y_max != 0.0 {
                    2.0_f64.mul_add(-(y_min + y_max), height)
                } else {
                    2.0_f64.mul_add(-overflow.top.max(overflow.bottom), height)
                };
            }
        }

        {
            self.option.apply(state, available_width, available_height);
        }

        let next_dimensions: Dimensions = state.platform.dimensions(&state.elements.floating);

        let size_opt = if width.approx_ne(next_dimensions.width, (0.0, 1))
            || height.approx_ne(next_dimensions.height, (0.0, 1))
        {
            Some(true)
        } else {
            None
        };
        let data = MiddlewareData {
            kind: self.kind(),
            size: size_opt,
            ..Default::default()
        };

        MiddlewareReturn {
            data,
            ..Default::default()
        }
    }
}
