// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::origin::HorizontalOrigin;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Down
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HorizontalDirection {
    Left,
    Right,
}

impl Default for HorizontalOrigin {
    fn default() -> Self {
        Self::Left
    }
}
