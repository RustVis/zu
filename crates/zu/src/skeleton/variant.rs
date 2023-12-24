// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

/// The type of content that will be rendered.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Variant {
    #[default]
    Text,
    Circular,
    Rect,
    Rounded,
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Text => "ZuSkeleton-text",
            Self::Circular => "ZuSkeleton-circular",
            Self::Rect => "ZuSkeleton-rect",
            Self::Rounded => "ZuSkeleton-rounded",
        }
    }
}
