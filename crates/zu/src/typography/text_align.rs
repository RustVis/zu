// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

/// Set the text-align on the component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    /// The inline contents are centered within the line box.
    Center,

    /// The same as left if direction is left-to-right and right if direction is right-to-left.
    Left,

    /// The same as right if direction is left-to-right and left if direction is right-to-left.
    Right,

    /// The inline contents are justified.
    Justify,
    Inherit,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Inherit
    }
}

impl CssClass for TextAlign {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Center => "ZuTypography-center",
            Self::Left => "ZuTypography-left",
            Self::Right => "ZuTypography-right",
            Self::Justify => "ZuTypography-justify",
            Self::Inherit => "",
        }
    }
}
