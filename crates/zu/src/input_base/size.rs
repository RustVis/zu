// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::size::Size;

#[must_use]
pub const fn css_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuInputBase-sizeSmall",
        _ => "",
    }
}

#[must_use]
pub const fn input_css_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuInputBase-inputSizeSmall",
        _ => "",
    }
}
