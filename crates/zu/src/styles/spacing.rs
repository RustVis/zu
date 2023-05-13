// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    None,
    XXSmall,
    XSmall,
    SmallNudge,
    Small,
    MiddleNudge,
    Middle,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
}

impl Spacing {
    #[must_use]
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::XXSmall => "xxs",
            Self::XSmall => "xs",
            Self::SmallNudge => "sNudge",
            Self::Small => "s",
            Self::MiddleNudge => "mNudge",
            Self::Middle => "m",
            Self::Large => "l",
            Self::XLarge => "xl",
            Self::XXLarge => "xxl",
            Self::XXXLarge => "xxxl",
        }
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::None
    }
}

impl CssValue for Spacing {
    fn css_value(&self) -> String {
        let s = self.to_str();
        format!("var(--zu-spacing-{s})")
    }
}
