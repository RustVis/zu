// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Default,
    Primary,
    Secondary,
    Inherit,
    Transparent,
}

impl Default for Color {
    fn default() -> Self {
        Self::Primary
    }
}

impl CssClass for Color {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Default => "ZuAppBar-colorDefault",
            Self::Primary => "ZuAppBar-colorPrimary",
            Self::Secondary => "ZuAppBar-colorSecondary",
            Self::Inherit => "ZuApPBar-colorInherit",
            Self::Transparent => "ZuAppBar-colorTransparent",
        }
    }
}
