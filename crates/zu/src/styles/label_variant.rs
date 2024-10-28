// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LabelVariant {
    Filled,
    Outlined,
    #[default]
    Standard,
}

impl LabelVariant {
    #[inline]
    #[must_use]
    pub fn is_contained(self) -> bool {
        self == Self::Filled || self == Self::Outlined
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SimpleLabelVariant {
    #[default]
    Filled,
    Outlined,
}
