// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::CssClass;

/// Defines the flex-direction style property.
///
/// It is applied for all screen sizes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl CssClass for FlexDirection {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Column => "ZuDirection-column",
            Self::ColumnReverse => "ZuDirection-columnReverse",
            Self::Row => "ZuDirection-row",
            Self::RowReverse => "ZuDirection-rowReverse",
        }
    }
}
