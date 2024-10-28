// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::CssClass;

/// Applies the theme typography styles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Body1,
    Body2,
    Button,
    Caption,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Overline,
    Subtitle1,
    Subtitle2,
    Inherit,
}

impl Variant {
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn as_component(&self) -> &'static str {
        match self {
            Self::Body1 => "p",
            Self::Body2 => "p",
            Self::Button => "span",
            Self::Caption => "span",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Overline => "span",
            Self::Subtitle1 => "h6",
            Self::Subtitle2 => "h6",
            Self::Inherit => "p",
        }
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Body1 => "ZuTypography-body1",
            Self::Body2 => "ZuTypography-body2",
            Self::Button => "ZuTypography-button",
            Self::Caption => "ZuTypography-caption",
            Self::H1 => "ZuTypography-h1",
            Self::H2 => "ZuTypography-h2",
            Self::H3 => "ZuTypography-h3",
            Self::H4 => "ZuTypography-h4",
            Self::H5 => "ZuTypography-h5",
            Self::H6 => "ZuTypography-h6",
            Self::Overline => "ZuTypography-overline",
            Self::Subtitle1 => "ZuTypography-subtitle1",
            Self::Subtitle2 => "ZuTypography-subtitle2",
            Self::Inherit => "",
        }
    }
}
