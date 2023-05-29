// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

/// The size of the component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    XSmall,
    Small,
    Middle,
    Large,
    XLarge,

    Str(String),
}

impl Default for Size {
    fn default() -> Self {
        Self::Middle
    }
}

impl CssClass for Size {
    fn css_class(&self) -> &'static str {
        match self {
            Self::XSmall => "ZuContainer-maxWidthXs",
            Self::Small => "ZuContainer-maxWidthSm",
            Self::Middle => "ZuContainer-maxWidthMd",
            Self::Large => "ZuContainer-maxWidthLg",
            Self::XLarge => "ZuContainer-maxWidthXl",
            Self::Str(_) => "",
        }
    }
}

#[must_use]
pub fn max_width_cls(size: &Option<Size>) -> &'static str {
    size.as_ref().map_or("", |size| size.css_class())
}
