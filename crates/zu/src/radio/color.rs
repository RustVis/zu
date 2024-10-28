// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::color::Color;

#[must_use]
pub const fn css_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuRadio-colorPrimary",
        Color::Secondary => "ZuRadio-colorSecondary",
        Color::Success => "ZuRadio-colorSuccess",
        Color::Info => "ZuRadio-colorInfo",
        Color::Warning => "ZuRadio-colorWarning",
        Color::Error => "ZuRadio-colorError",
        Color::Default => "ZuRadio-colorDefault",
        Color::Inherit => "",
    }
}
