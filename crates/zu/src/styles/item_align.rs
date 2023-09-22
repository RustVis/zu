// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

/// Defines the align-items style property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAlign {
    Center,
    FlexStart,
}

impl Default for ItemAlign {
    fn default() -> Self {
        Self::Center
    }
}
