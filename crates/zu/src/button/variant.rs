// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Contained,
    Outlined,
    Text,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Text
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Contained => "ZuButton-contained",
            Self::Outlined => "ZuButton-outlined",
            Self::Text => "ZuButton-text",
        }
    }
}