// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Defines the flex-direction style property.
///
/// It is applied for all screen sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Column
    }
}
