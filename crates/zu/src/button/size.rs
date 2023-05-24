// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Self::Small
    }
}

impl CssClass for Size {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Small => "ZuButton-sizeSmall",
            Self::Medium => "ZuButton-sizeMedium",
            Self::Large => "ZuButton-sizeLarge",
        }
    }
}

impl Size {
    #[must_use]
    pub const fn icon_class(self) -> &'static str {
        match self {
            Self::Small => "ZuButton-iconSizeSmall",
            Self::Medium => "ZuButton-iconSizeMedium",
            Self::Large => "ZuButton-iconSizeLarge",
        }
    }
}
