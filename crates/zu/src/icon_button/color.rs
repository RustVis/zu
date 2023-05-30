// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::color::Color;

#[must_use]
pub const fn css_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuIconButton-colorPrimary",
        Color::Secondary => "ZuIconButton-colorSecondary",
        Color::Error => "ZuIconButton-colorError",
        Color::Info => "ZuIconButton-colorInfo",
        Color::Warning => "ZuIconButton-colorWarning",
        Color::Success => "ZuIconButton-colorSuccess",
        Color::Inherit => "ZuIconButton-colorInherit",
        Color::Default => "",
    }
}
