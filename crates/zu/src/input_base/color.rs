// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::color::Color;

#[must_use]
pub const fn css_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuInputBase-colorPrimary",
        Color::Secondary => "ZuInputBase-colorSecondary",
        Color::Success => "ZuInputBase-colorSuccess",
        Color::Info => "ZuInputBase-colorInfo",
        Color::Warning => "ZuInputBase-colorWarning",
        Color::Error => "ZuInputBase-colorError",
        Color::Default => "ZuInputBase-colorDefault",
        Color::Inherit => "",
    }
}
