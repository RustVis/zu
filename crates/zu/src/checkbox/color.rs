// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::color::Color;

#[must_use]
pub const fn css_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuCheckbox-colorPrimary",
        Color::Secondary => "ZuCheckbox-colorSecondary",
        Color::Success => "ZuCheckbox-colorSuccess",
        Color::Info => "ZuCheckbox-colorInfo",
        Color::Warning => "ZuCheckbox-colorWarning",
        Color::Error => "ZuCheckbox-colorError",
        Color::Default => "ZuCheckbox-colorDefault",
        Color::Inherit => "",
    }
}
