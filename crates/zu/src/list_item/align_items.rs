// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Defines the align-items style property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    Center,
    FlexStart,
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Center
    }
}
