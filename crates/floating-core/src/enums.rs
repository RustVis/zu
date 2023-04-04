// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignedPlacement {
    TopStart,
    TopEnd,

    RightStart,
    RightEnd,

    BottomStart,
    BottomEnd,

    LeftStart,
    LeftEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    // Top
    TopStart,
    Top,
    TopEnd,

    // Right
    RightStart,
    Right,
    RightEnd,

    // Bottom
    BottomStart,
    Bottom,
    BottomEnd,

    // Left
    LeftStart,
    Left,
    LeftEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Absolute,
    Fixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Width,
    Height,
}
