// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom,
    Start,
    End,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalPosition {
    Left,
    #[default]
    Right,
    Alternate,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PositionValue {
    pub top: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
    pub right: Option<i32>,
}
