// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
    Always,
    Hover,
    None,
}

impl Default for Underline {
    fn default() -> Self {
        Self::Always
    }
}

impl CssClass for Underline {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Always => "ZuLink-underlineAlways",
            Self::Hover => "ZuLink-underlineHover",
            Self::None => "ZuLink-underlineNone",
        }
    }
}
