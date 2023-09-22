// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Position {
    Bottom,
    Top,
    Static,
}

impl Default for Position {
    fn default() -> Self {
        Self::Bottom
    }
}
