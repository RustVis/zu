// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::size::Size;

#[must_use]
pub fn css_value(size: Size) -> String {
    // TODO(Shaohua): Read from css.
    let size = match size {
        Size::XSmall => 8,
        Size::Small => 12,
        Size::Middle => 14,
        Size::Large => 18,
        Size::XLarge => 24,
        Size::Num(num) => num,
    };
    format!("width: {size}px; height: {size}px")
}
