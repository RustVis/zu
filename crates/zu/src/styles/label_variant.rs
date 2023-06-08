// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelVariant {
    Filled,
    Outlined,
    Standard,
}

impl Default for LabelVariant {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelVariantNoStd {
    Filled,
    Outlined,
}

impl Default for LabelVariantNoStd {
    fn default() -> Self {
        Self::Filled
    }
}
