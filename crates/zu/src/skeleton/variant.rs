// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

/// The type of content that will be rendered.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Variant {
    Text,
    Circle,
    Rect,
    Rounded,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Text
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Text => "ZuSkeleton-text",
            Self::Circle => "ZuSkeleton-circle",
            Self::Rect => "ZuSkeleton-rect",
            Self::Rounded => "ZuSkeleton-rounded",
        }
    }
}
