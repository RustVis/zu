// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed byLesser General Public License that can be found
// in the LICENSE file.

use crate::styles::color::Color;

pub const fn class_name(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuTypography-colorPrimary",
        Color::Secondary => "ZuTypography-colorSecondary",
        Color::Success => "ZuTypography-colorSuccess",
        Color::Info => "ZuTypography-colorInfo",
        Color::Warning => "ZuTypography-colorWarning",
        Color::Error => "ZuTypography-colorError",
        Color::Inherit => "ZuTypography-colorInherit",
        Color::Default => "ZuTypography-colorDefault",
    }
}
