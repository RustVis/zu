// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Margin {
    Dense,
    None,
    Normal,
}

impl Default for Margin {
    fn default() -> Self {
        Self::None
    }
}

impl CssClass for Margin {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Dense => "ZuFormControl-marginDense",
            Self::Normal => "ZuFormControl-marginNormal",
            Self::None => "",
        }
    }
}
