// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

use crate::traits::{AxisTrait, LengthTrait};
use crate::types::{Alignment, Axis, Coords, ElementRects, Length, Placement, Side};

#[must_use]
pub fn compute_coords_from_placement(
    element_rects: &ElementRects,
    placement: Placement,
    rtl: bool,
) -> Coords {
    let reference = &element_rects.reference;
    let floating = &element_rects.floating;

    let common_x = reference.x + reference.width / 2.0 - floating.width / 2.0;
    let common_y = reference.y + reference.height / 2.0 - floating.height / 2.0;
    let main_axis = placement.main_axis();
    let length: Length = main_axis.into();
    let common_align = reference.length(length) / 2.0 - floating.length(length) / 2.0;
    let side = placement.side();
    let is_vertical = main_axis == Axis::X;

    let mut coords: Coords = match side {
        Side::Top => Coords::new(common_x, reference.y - floating.height),
        Side::Bottom => Coords::new(common_x, reference.y + reference.height),
        Side::Right => Coords::new(reference.x + reference.width, common_y),
        Side::Left => Coords::new(reference.x - floating.width, common_y),
    };

    let factor = if rtl && is_vertical { -1.0 } else { 1.0 };

    match placement.alignment() {
        Some(Alignment::Start) => {
            let val = common_align.mul_add(-factor, coords.axis(main_axis));
            coords.set_axis(main_axis, val);
        }
        Some(Alignment::End) => {
            let val = common_align.mul_add(factor, coords.axis(main_axis));
            coords.set_axis(main_axis, val);
        }
        None => (),
    }

    coords
}
