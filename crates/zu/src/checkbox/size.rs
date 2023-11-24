// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::size::Size;
use crate::svg_icon::FontSize;

#[inline]
#[must_use]
pub const fn size_to_font_size(size: Size) -> FontSize {
    match size {
        Size::Small => FontSize::Small,
        Size::Medium => FontSize::Medium,
        Size::Large => FontSize::Large,
    }
}
