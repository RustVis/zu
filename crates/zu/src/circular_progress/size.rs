// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

/// The size of the component.
///
/// If using a number, the pixel unit is assumed.
/// If using a string, you need to provide the CSS unit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Num(i32),
    Str(String),
}

impl Size {
    #[must_use]
    pub fn css_value(&self) -> String {
        match self {
            Self::Num(num) => format!("width: {num}px; height: {num}px"),
            Self::Str(s) => format!("width: {s}; height: {s};"),
        }
    }
}
