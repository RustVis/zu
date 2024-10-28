// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    #[default]
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

impl Spacing {
    #[must_use]
    pub fn css_value(&self) -> String {
        let s = self.to_str();
        format!("var(--zu-spacing-{s})")
    }
}
