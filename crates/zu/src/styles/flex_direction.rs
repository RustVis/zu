// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::CssClass;

/// Defines the flex-direction style property.
///
/// It is applied for all screen sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Column
    }
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
