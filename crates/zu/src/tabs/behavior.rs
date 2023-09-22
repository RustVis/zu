// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ButtonBehavior {
    Auto,
    True,
    False,
}

impl Default for ButtonBehavior {
    fn default() -> Self {
        Self::Auto
    }
}
