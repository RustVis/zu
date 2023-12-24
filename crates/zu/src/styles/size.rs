// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::svg_icon::FontSize;

/// Size of component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    #[default]
    Small,
    Medium,
    Large,
}

impl Size {
    #[must_use]
    pub const fn to_font_size(self) -> FontSize {
        match self {
            Self::Small => FontSize::Small,
            Self::Medium => FontSize::Medium,
            Self::Large => FontSize::Large,
        }
    }
}

/// The max-width of the component.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum MaxWidth {
    XSmall,
    Small,
    #[default]
    Middle,
    Large,
    XLarge,

    Str(String),
}
