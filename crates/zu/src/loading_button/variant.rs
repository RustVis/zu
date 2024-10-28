// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::button_variant::ButtonVariant;

pub const fn root_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Contained => "ZuLoadingButton-contained",
        ButtonVariant::Outlined => "ZuLoadingButton-outlined",
        ButtonVariant::Text => "ZuLoadingButton-text",
    }
}

pub const fn indicator_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Contained => "ZuLoadingButton-loadingIndicatorContained",
        ButtonVariant::Outlined => "ZuLoadingButton-loadingIndicatorOutlined",
        ButtonVariant::Text => "ZuLoadingButton-loadingIndicatorText",
    }
}
