// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

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
