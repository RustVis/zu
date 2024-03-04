// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Provides data to change the size of a floating element.
//!
//! This is useful to ensure the floating element isn’t too big to fit in the viewport
//! (or more specifically, its clipping context), especially when a maximum size isn’t specified.
//! It also allows matching the width/height of the reference element.

use float_cmp::ApproxEq;
use std::fmt;
use std::fmt::Formatter;

use crate::detect_overflow::{detect_overflow, DetectOverflowOption};
use crate::middleware::{Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState};
use crate::middlewares::shift::ShiftMiddlewareData;
use crate::types::{Alignment, Axis, Dimensions, Rect, Side, SideTrait};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SizeMiddlewareData {
    pub size_opt: bool,
}

impl SizeMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Size::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

pub trait SizeOptionTrait {
    // TODO(Shaohua): Returns new coords.
    fn apply(&self, state: &MiddlewareState, available_width: f64, available_height: f64);
}

pub struct Size {
    pub option: Box<dyn SizeOptionTrait>,
    pub detect_overflow_option: DetectOverflowOption,
}

impl fmt::Debug for Size {
    #[allow(clippy::missing_fields_in_debug)]
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Size {
    pub const NAME: &'static str = "size";
}

impl Middleware for Size {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, state: &MiddlewareState) -> MiddlewareReturn {
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

        if ShiftMiddlewareData::from(state.middleware_data).is_none() && alignment.is_none() {
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
        let size_opt = width.approx_ne(next_dimensions.width, (0.0, 1))
            || height.approx_ne(next_dimensions.height, (0.0, 1));

        let boxed_size_data = Box::new(SizeMiddlewareData { size_opt });
        let data = MiddlewareData::with_value(Self::NAME, boxed_size_data);
        MiddlewareReturn::from_data(data)
    }
}
