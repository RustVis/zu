// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

/// The color of the component. It supports both default and custom theme colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Error,
    Info,
    Warning,
    Success,
    Inherit,
    Default,
}

impl Default for Color {
    fn default() -> Self {
        Self::Primary
    }
}

impl Color {
    #[must_use]
    pub fn text_color(&self) -> String {
        format!("color: {}", self.css_value())
    }

    #[must_use]
    pub fn bg_color(&self) -> String {
        format!("background-color: {}", self.css_value())
    }
}

impl CssValue for Color {
    fn css_value(&self) -> String {
        match self {
            Self::Primary => "var(--zu-palette-primary-main)".to_owned(),
            Self::Secondary => "var(--zu-palette-secondary-main)".to_owned(),
            Self::Error => "var(--zu-palette-error-main)".to_owned(),
            Self::Info => "var(--zu-palette-info-main)".to_owned(),
            Self::Warning => "var(--zu-palette-warning-main)".to_owned(),
            Self::Success => "var(--zu-palette-success-main)".to_owned(),
            Self::Inherit | Self::Default => String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicColor {
    Primary,
    Secondary,
}

impl Default for BasicColor {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardColor {
    Standard,
    Primary,
    Secondary,
}

impl Default for StandardColor {
    fn default() -> Self {
        Self::Standard
    }
}
