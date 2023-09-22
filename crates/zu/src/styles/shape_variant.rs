// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeVariant {
    Circular,
    Rounded,
    Square,
}

impl Default for ShapeVariant {
    fn default() -> Self {
        Self::Circular
    }
}
