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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SideObject {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClientRectObject {
    pub rect: Rect,
    pub side: SideObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementContext {
    Reference,
    Floating,
}
