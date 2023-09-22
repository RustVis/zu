// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlurOption {
    False,
    True,
    Mouse,
    Touch,
}

impl Default for BlurOption {
    fn default() -> Self {
        Self::False
    }
}
