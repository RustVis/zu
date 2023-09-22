// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::anchor_origin::Horizontal;

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

impl Default for Horizontal {
    fn default() -> Self {
        Self::Left
    }
}
