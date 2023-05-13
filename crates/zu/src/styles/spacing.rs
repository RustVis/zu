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

impl Default for Spacing {
    fn default() -> Self {
        Self::None
    }
}

impl CssValue for Spacing {
    fn css_value(&self) -> String {
        match self {
            Self::None => "var(--zu-spacing-none",
            Self::XXSmall => "var(--zu-spacing-xxs)",
            Self::XSmall => "var(--zu-spacing-xs)",
            Self::SmallNudge => "var(--zu-spacing-sNudge)",
            Self::Small => "var(--zu-spacing-s)",
            Self::MiddleNudge => "var(--zu-spacing-mNudge)",
            Self::Middle => "var(--zu-spacing-m)",
            Self::Large => "var(--zu-spacing-l)",
            Self::XLarge => "var(--zu-spacing-xl)",
            Self::XXLarge => "var(--zu-spacing-xxl)",
            Self::XXXLarge => "var(--zu-spacing-xxxl)",
        }
        .to_owned()
    }
}
