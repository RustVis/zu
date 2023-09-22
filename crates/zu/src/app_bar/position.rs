// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Absolute,
    Fixed,
    Relative,
    Static,
    Sticky,
}

impl Default for Position {
    fn default() -> Self {
        Self::Fixed
    }
}

impl CssClass for Position {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Absolute => "ZuAppBar-positionAbsolute",
            Self::Fixed => "ZuAppBar-positionFixed",
            Self::Relative => "ZuAppBar-positionRelative",
            Self::Static => "ZuAppBar-positionStatic",
            Self::Sticky => "ZuAppBar-positionSticky",
        }
    }
}
