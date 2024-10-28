// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::color::Color;

pub const fn color_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuSwitch-colorPrimary",
        Color::Secondary => "ZuSwitch-colorSecondary",
        Color::Success => "ZuSwitch-colorSuccess",
        Color::Info => "ZuSwitch-colorInfo",
        Color::Warning => "ZuSwitch-colorWarning",
        Color::Error => "ZuSwitch-colorError",
        Color::Inherit => "ZuSwitch-colorInherit",
        Color::Default => "",
    }
}
