// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
    #[default]
    Always,
    Hover,
    None,
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
