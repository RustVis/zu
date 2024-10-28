// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::color::Color;

pub const fn css_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuCircularProgress-colorPrimary",
        Color::Secondary => "ZuCircularProgress-colorSecondary",
        _ => "",
    }
}
