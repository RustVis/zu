// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

/// The size of the component.
///
/// If using a number, the pixel unit is assumed.
/// If using a string, you need to provide the CSS unit, e.g '3rem'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SizeVariant {
    Num(i32),
    Str(String),
}

impl Default for SizeVariant {
    fn default() -> Self {
        Self::Num(40)
    }
}

impl CssValue for SizeVariant {
    fn css_value(&self) -> String {
        match self {
            Self::Num(num) => format!("{num}px"),
            Self::Str(str) => str.clone(),
        }
    }
}
