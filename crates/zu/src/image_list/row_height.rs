// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RowHeight {
    Auto,
    Num(i32),
}

impl Default for RowHeight {
    fn default() -> Self {
        Self::Auto
    }
}
