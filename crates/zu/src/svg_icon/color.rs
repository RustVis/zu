// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

/// Another color variant with `Disabled` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Inherit,
    Action,
    Disabled,
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Error,
}

impl Default for Color {
    fn default() -> Self {
        Self::Inherit
    }
}

impl CssClass for Color {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Inherit => "",
            Self::Action => "ZuSvgIcon-colorAction",
            Self::Disabled => "ZuSvgIcon-colorDisabled",
            Self::Primary => "ZuSvgIcon-colorPrimary",
            Self::Secondary => "ZuSvgIcon-colorSecondary",
            Self::Error => "ZuSvgIcon-colorError",
            Self::Info => "ZuSvgIcon-colorInfo",
            Self::Success => "ZuSvgIcon-colorSuccess",
            Self::Warning => "ZuSvgIcon-colorWarning",
        }
    }
}
