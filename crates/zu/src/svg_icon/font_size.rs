// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontSize {
    Inherit,
    Small,
    Medium,
    Large,
    Custom(String),
}

impl Default for FontSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl CssClass for FontSize {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Inherit => "ZuSvgIcon-fontSizeInherit",
            Self::Small => "ZuSvgIcon-fontSizeSmall",
            Self::Medium => "ZuSvgIcon-fontSizeMedium",
            Self::Large => "ZuSvgIcon-fontSizeLarge",
            Self::Custom(_) => "",
        }
    }
}
