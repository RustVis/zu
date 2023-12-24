// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Overlap {
    Circular,
    #[default]
    Rect,
}

impl CssClass for Overlap {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Circular => "ZuBadge-overlapCircular",
            Self::Rect => "ZuBadge-overlapRect",
        }
    }
}
