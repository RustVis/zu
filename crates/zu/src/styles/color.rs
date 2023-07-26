// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

/// The color of the component. It supports both default and custom theme colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Error,
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

    #[must_use]
    pub const fn capitalize(self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Secondary => "Secondary",
            Self::Success => "Success",
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Inherit => "Inherit",
            Self::Default => "Default",
        }
    }
}

impl CssValue for Color {
    fn css_value(&self) -> &'static str {
        match self {
            Self::Primary => "var(--zu-palette-primary-main)",
            Self::Secondary => "var(--zu-palette-secondary-main)",
            Self::Error => "var(--zu-palette-error-main)",
            Self::Info => "var(--zu-palette-info-main)",
            Self::Warning => "var(--zu-palette-warning-main)",
            Self::Success => "var(--zu-palette-success-main)",
            Self::Inherit | Self::Default => "",
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
pub enum InheritColor {
    Primary,
    Secondary,
    Inherit,
}

impl Default for InheritColor {
    fn default() -> Self {
        Self::Primary
    }
}
