// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub use crate::styles::color::Color;

pub const fn root_class(color: &Color) -> &'static str {
    match color {
        Color::Primary => "ZuLinearProgress-colorPrimary",
        Color::Secondary => "ZuLinearProgress-colorSecondary",
        Color::Inherit => "ZuLinearProgress-colorInherit",
        _ => "",
    }
}

#[must_use]
pub const fn dashed_class(color: &Color) -> &'static str {
    match color {
        Color::Primary => "ZuLinearProgress-dashColorPrimary",
        Color::Secondary => "ZuLinearProgress-dashColorSecondary",
        _ => "",
    }
}

#[must_use]
pub const fn bar_class(color: &Color) -> &'static str {
    match color {
        Color::Primary => "ZuLinearProgress-barColorPrimary",
        Color::Secondary => "ZuLinearProgress-barColorSecondary",
        _ => "",
    }
}
