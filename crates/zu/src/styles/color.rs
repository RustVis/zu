// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

/// The color of the component. It supports both default and custom theme colors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorVariant {
    Primary,
    Secondary,
    Error,
    Info,
    Warning,
    Success,
    Inherit,
    Custom(String),
}

impl Default for ColorVariant {
    fn default() -> Self {
        Self::Primary
    }
}

impl CssValue for ColorVariant {
    fn css_value(&self) -> String {
        match self {
            Self::Primary => "var(--zu-palette-primary-main)".to_owned(),
            Self::Secondary => "var(--zu-palette-secondary-main)".to_owned(),
            Self::Error => "var(--zu-palette-error-main)".to_owned(),
            Self::Info => "var(--zu-palette-info-main)".to_owned(),
            Self::Warning => "var(--zu-palette-warning-main)".to_owned(),
            Self::Success => "var(--zu-palette-success-main)".to_owned(),
            Self::Inherit => String::new(),
            Self::Custom(color) => color.clone(),
        }
    }
}