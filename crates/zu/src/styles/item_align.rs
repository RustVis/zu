// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

/// Defines the align-items style property.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ItemAlign {
    #[default]
    Center,
    FlexStart,
}
