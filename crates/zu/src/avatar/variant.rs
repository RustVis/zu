// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::shape_variant::ShapeVariant;

#[must_use]
pub const fn css_class(shape_variant: ShapeVariant) -> &'static str {
    match shape_variant {
        ShapeVariant::Circle => "ZuAvatar-circle",
        ShapeVariant::Rounded => "ZuAvatar-rounded",
        ShapeVariant::Square => "ZuAvatar-square",
    }
}
