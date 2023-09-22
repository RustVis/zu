// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::size::Size;

#[must_use]
pub const fn root_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuLoadingButton-sizeSmall",
        Size::Medium => "ZuLoadingButton-sizeMedium",
        Size::Large => "ZuLoadingButton-sizeLarge",
    }
}

#[must_use]
pub const fn icon_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuLoadingButton-iconSizeSmall",
        Size::Medium => "ZuLoadingButton-iconSizeMedium",
        Size::Large => "ZuLoadingButton-iconSizeLarge",
    }
}

#[must_use]
pub const fn indicator_class(size: Size) -> &'static str {
    match size {
        Size::Small => "ZuLoadingButton-loadingIndicatorSizeSmall",
        Size::Medium => "ZuLoadingButton-loadingIndicatorSizeMedium",
        Size::Large => "ZuLoadingButton-loadingIndicatorSizeLarge",
    }
}
