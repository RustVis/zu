// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

/// Size of component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Self::Small
    }
}

/// The max-width of the component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaxWidth {
    XSmall,
    Small,
    Middle,
    Large,
    XLarge,

    Str(String),
}

impl Default for MaxWidth {
    fn default() -> Self {
        Self::Middle
    }
}
