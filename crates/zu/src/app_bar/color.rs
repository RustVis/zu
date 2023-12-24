// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Default,
    #[default]
    Primary,
    Secondary,
    Inherit,
    Transparent,
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
