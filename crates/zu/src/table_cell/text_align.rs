// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

/// Set the text-align on the component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Center,
    Left,
    Right,
    Justify,
}

impl CssClass for TextAlign {
    fn css_class(&self) -> &'static str {
        todo!()
    }
}
