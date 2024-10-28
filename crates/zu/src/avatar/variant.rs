// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::shape_variant::ShapeVariant;

#[must_use]
pub const fn css_class(shape_variant: ShapeVariant) -> &'static str {
    match shape_variant {
        ShapeVariant::Circular => "ZuAvatar-circular",
        ShapeVariant::Rounded => "ZuAvatar-rounded",
        ShapeVariant::Square => "ZuAvatar-square",
    }
}
