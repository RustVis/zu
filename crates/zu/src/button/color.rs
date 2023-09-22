// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::color::Color;

pub const fn color_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuButton-colorPrimary",
        Color::Secondary => "ZuButton-colorSecondary",
        Color::Success => "ZuButton-colorSuccess",
        Color::Info => "ZuButton-colorInfo",
        Color::Warning => "ZuButton-colorWarning",
        Color::Error => "ZuButton-colorError",
        Color::Inherit => "ZuButton-colorInherit",
        Color::Default => "",
    }
}
