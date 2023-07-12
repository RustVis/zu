// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::color::DisabledColor;

pub const fn css_class(color: DisabledColor) -> &'static str {
    match color {
        DisabledColor::Inherit => "",
        DisabledColor::Action => "ZuSvgIcon-colorAction",
        DisabledColor::Disabled => "ZuSvgIcon-colorDisabled",
        DisabledColor::Primary => "ZuSvgIcon-colorPrimary",
        DisabledColor::Secondary => "ZuSvgIcon-colorSecondary",
        DisabledColor::Error => "ZuSvgIcon-colorError",
        DisabledColor::Info => "ZuSvgIcon-colorInfo",
        DisabledColor::Success => "ZuSvgIcon-colorSuccess",
        DisabledColor::Warning => "ZuSvgIcon-colorWarning",
    }
}
