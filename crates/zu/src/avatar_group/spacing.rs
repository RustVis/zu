// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    Small,
    Medium,
    Num(i32),
}

impl Default for Spacing {
    fn default() -> Self {
        Self::Medium
    }
}

impl Spacing {
    #[must_use]
    pub const fn space(self) -> Option<i32> {
        match self {
            Self::Small => Some(-16),
            Self::Medium => None,
            Self::Num(n) => Some(-n),
        }
    }
}
