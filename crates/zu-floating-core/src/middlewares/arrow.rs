// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

//! Provides positioning data for an arrow element (triangle or caret) inside the floating element,
//! such that it appears to be pointing toward the center of the reference element.
//!
//! This is useful to add a visual cue to the floating element about which element
//! it is referring to.

use float_cmp::ApproxEq;
use std::fmt;
use std::rc::Rc;

use crate::middleware::{Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState};
use crate::platform::Element;
use crate::types::{
    Axis, AxisTrait, Length, LengthTrait, Padding, PartialCoords, Side, SideObject, SideTrait,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArrowMiddlewareData {
    pub coords: PartialCoords,
    pub center_offset: f64,
}

impl ArrowMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Arrow::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

#[derive(Clone)]
pub struct ArrowOption {
    /// This is the arrow element to be positioned, which must be a child of the floating element.
    pub element: Rc<dyn Element>,

    /// The padding between the arrow element and the floating element edges.
    ///
    /// Useful when the floating element has round corners.
    pub padding: Padding,
}

impl fmt::Debug for ArrowOption {
    // TODO(Shaohua): Add element field.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrowOption")
            .field("padding", &self.padding)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub option: ArrowOption,
}

impl Arrow {
    pub const NAME: &'static str = "arrow";
}

impl Middleware for Arrow {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, state: &MiddlewareState) -> MiddlewareReturn {
        let coords = &state.coords;
        let rects = &state.rects;
        let platform = &state.platform;
        let placement = &state.placement;

        let padding_object: SideObject = self.option.padding.clone().into();
        let axis = placement.main_axis();
        let length: Length = axis.into();
        let arrow_dimensions = platform.dimensions(&self.option.element);
        let is_y_axis = axis == Axis::Y;
        let min_prop = if is_y_axis { Side::Top } else { Side::Left };
        let max_prop = if is_y_axis { Side::Bottom } else { Side::Right };
        let client_prop = if is_y_axis {
            Length::Height
        } else {
            Length::Width
        };

        let end_diff = rects.reference.length(length) + rects.reference.axis(axis)
            - coords.axis(axis)
            - rects.floating.length(length);
        let start_diff = coords.axis(axis) - rects.reference.axis(axis);

        let offset_parent = platform.offset_parent(&self.option.element);
        let client_size: f64 =
            offset_parent.map_or(0.0, |offset_parent| offset_parent.length(client_prop));

        // TODO(Shaohua): Dom related parent

        let center_to_reference: f64 = end_diff / 2.0 - start_diff / 2.0;
        let min: f64 = padding_object.side(min_prop);
        let max: f64 =
            client_size - arrow_dimensions.length(length) - padding_object.side(max_prop);
        let center: f64 =
            client_size / 2.0 - arrow_dimensions.length(length) / 2.0 + center_to_reference;
        let offset: f64 = center.clamp(min, max);

        let expected_offset: f64 = if center < min {
            rects.reference.length(length) / 2.0
                - padding_object.side(min_prop)
                - arrow_dimensions.length(length) / 2.0
        } else {
            rects.reference.length(length) / 2.0
                - padding_object.side(max_prop)
                - arrow_dimensions.length(length) / 2.0
        };
        let should_add_offset: bool = placement.alignment().is_some()
            && center.approx_ne(offset, (0.0, 1))
            && expected_offset < 0.0;

        let alignment_offset: f64 = if should_add_offset {
            if center < min {
                min - center
            } else {
                max - center
            }
        } else {
            0.0
        };

        let arrow_data = Box::new(ArrowMiddlewareData {
            coords: PartialCoords::new(axis, offset),
            center_offset: center - offset,
        });
        let data = MiddlewareData::with_value(Self::NAME, arrow_data);
        MiddlewareReturn {
            coords: PartialCoords::new(axis, coords.axis(axis) - alignment_offset),
            data,
            ..Default::default()
        }
    }
}
