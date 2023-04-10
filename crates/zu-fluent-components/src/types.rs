// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementSize {
    Tiny,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Huge,
}

impl Default for ElementSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativePosition {
    Above,
    Below,
    Before,
    After,
}

impl Default for RelativePosition {
    fn default() -> Self {
        Self::After
    }
}