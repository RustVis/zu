// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::size::MaxWidth;

#[must_use]
pub const fn css_class(size: &Option<MaxWidth>) -> &'static str {
    match size {
        Some(MaxWidth::XSmall) => "ZuContainer-maxWidthXs",
        Some(MaxWidth::Small) => "ZuContainer-maxWidthSm",
        Some(MaxWidth::Middle) => "ZuContainer-maxWidthMd",
        Some(MaxWidth::Large) => "ZuContainer-maxWidthLg",
        Some(MaxWidth::XLarge) => "ZuContainer-maxWidthXl",
        _ => "",
    }
}
