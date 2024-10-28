// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::size::Size;

#[must_use]
pub const fn css_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuButton-sizeSmall",
        Size::Medium => "ZuButton-sizeMedium",
        Size::Large => "ZuButton-sizeLarge",
    }
}

#[must_use]
pub const fn icon_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuButton-iconSizeSmall",
        Size::Medium => "ZuButton-iconSizeMedium",
        Size::Large => "ZuButton-iconSizeLarge",
    }
}
