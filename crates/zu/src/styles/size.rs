// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// The size of the component.
///
/// If using a number, the pixel unit is assumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SizeVariant {
    Tiny,
    Small,
    Regular,
    Large,
    XLarge,

    Num(i32),
}

impl Default for SizeVariant {
    fn default() -> Self {
        Self::Regular
    }
}
