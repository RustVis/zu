// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::button_variant::ButtonVariant;

pub const fn css_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Contained => "ZuButton-contained",
        ButtonVariant::Outlined => "ZuButton-outlined",
        ButtonVariant::Text => "ZuButton-text",
    }
}
