// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    #[default]
    FullWidth,
    Inset,
    Middle,
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::FullWidth => "ZuDivider-fullWidth",
            Self::Inset => "ZuDivider-inset",
            Self::Middle => "ZuDivider-middle",
        }
    }
}
