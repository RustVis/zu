// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Button,
    Reset,
    Submit,
}

impl CssValue for ButtonType {
    #[inline]
    #[must_use]
    fn css_value(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Reset => "reset",
            Self::Submit => "submit",
        }
    }
}
