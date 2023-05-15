// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Default,
    Primary,
    Secondary,
    Error,
    Info,
    Warning,
    Success,
    Custom(String),
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

impl CssClass for Color {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Primary => "ZuBadge-colorPrimary",
            Self::Secondary => "ZuBadge-colorSecondary",
            Self::Error => "ZuBadge-colorError",
            Self::Info => "ZuBadge-colorInfo",
            Self::Warning => "ZuBadge-colorWarning",
            Self::Success => "ZuBadge-colorSuccess",
            Self::Default | Self::Custom(_) => "",
        }
    }
}
