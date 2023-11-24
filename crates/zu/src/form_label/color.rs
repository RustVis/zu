// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::color::Color;

pub const fn color_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuFormLabel-colorPrimary",
        Color::Secondary => "ZuFormLabel-colorSecondary",
        Color::Success => "ZuFormLabel-colorSuccess",
        Color::Info => "ZuFormLabel-colorInfo",
        Color::Warning => "ZuFormLabel-colorWarning",
        Color::Error => "ZuFormLabel-colorError",
        Color::Inherit => "ZuFormLabel-colorInherit",
        Color::Default => "",
    }
}
