// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Dot,
    #[default]
    Standard,
}

impl Variant {
    #[must_use]
    pub const fn radius(&self) -> i32 {
        match self {
            Self::Dot => 4,
            Self::Standard => 10,
        }
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Dot => "ZuBadge-dot",
            Self::Standard => "ZuBadge-standard",
        }
    }
}
