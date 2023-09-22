// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollType {
    Body,
    Paper,
}

impl Default for ScrollType {
    fn default() -> Self {
        Self::Paper
    }
}
