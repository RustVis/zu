// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssValue;

/// Set the text-align on the component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    /// The inline contents are centered within the line box.
    Center,

    /// The same as left if direction is left-to-right and right if direction is right-to-left.
    Start,

    /// The same as right if direction is left-to-right and left if direction is right-to-left.
    End,

    /// The inline contents are justified.
    Justify,
    Inherit,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Inherit
    }
}

impl CssValue for TextAlign {
    fn css_value(&self) -> String {
        match self {
            Self::Center => "center".to_owned(),
            Self::Start => "start".to_owned(),
            Self::End => "end".to_owned(),
            Self::Justify => "justify".to_owned(),
            Self::Inherit => "inherit".to_owned(),
        }
    }
}
