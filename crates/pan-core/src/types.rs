// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use crate::traits::{Middleware, Platform};

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

impl From<AlignedPlacement> for Alignment {
    fn from(val: AlignedPlacement) -> Self {
        match val {
            AlignedPlacement::TopStart
            | AlignedPlacement::RightStart
            | AlignedPlacement::BottomStart
            | AlignedPlacement::LeftStart => Self::Start,
            AlignedPlacement::TopEnd
            | AlignedPlacement::RightEnd
            | AlignedPlacement::BottomEnd
            | AlignedPlacement::LeftEnd => Self::End,
        }
    }
}

impl AlignedPlacement {
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::TopStart => Self::TopEnd,
            Self::TopEnd => Self::TopStart,
            Self::RightStart => Self::RightEnd,
            Self::RightEnd => Self::RightStart,
            Self::BottomStart => Self::BottomEnd,
            Self::BottomEnd => Self::BottomStart,
            Self::LeftStart => Self::LeftEnd,
            Self::LeftEnd => Self::LeftStart,
        }
    }
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

impl Default for Placement {
    fn default() -> Self {
        Self::TopStart
    }
}

impl Placement {
    #[must_use]
    pub const fn side(self) -> Side {
        match self {
            Self::TopStart | Self::Top | Self::TopEnd => Side::Top,
            Self::RightStart | Self::Right | Self::RightEnd => Side::Right,
            Self::BottomStart | Self::Bottom | Self::BottomEnd => Side::Bottom,
            Self::LeftStart | Self::Left | Self::LeftEnd => Side::Left,
        }
    }

    /// Get opposite placement.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::TopStart => Self::BottomStart,
            Self::Top => Self::Bottom,
            Self::TopEnd => Self::BottomEnd,

            Self::RightStart => Self::LeftStart,
            Self::Right => Self::Left,
            Self::RightEnd => Self::LeftEnd,

            Self::BottomStart => Self::TopStart,
            Self::Bottom => Self::Top,
            Self::BottomEnd => Self::TopEnd,

            Self::LeftStart => Self::RightStart,
            Self::Left => Self::Right,
            Self::LeftEnd => Self::RightEnd,
        }
    }
}

impl From<Placement> for Side {
    fn from(placement: Placement) -> Self {
        placement.side()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    Absolute,
    Fixed,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Absolute
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    /// Get cross axis
    #[inline]
    #[must_use]
    pub const fn cross(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Width,
    Height,
}

impl From<Axis> for Length {
    fn from(axis: Axis) -> Self {
        match axis {
            Axis::X => Self::Width,
            Axis::Y => Self::Height,
        }
    }
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
pub struct PartialCoords {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArrowMiddlewareData {
    pub coords: PartialCoords,
    pub center_offset: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Overflow {
    pub placement: Placement,
    pub overflows: Vec<f64>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AutoPlacementMiddlewareData {
    pub index: Option<usize>,
    pub overflows: Vec<Overflow>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FlipMiddlewareData {
    pub index: Option<usize>,
    pub overflows: Vec<Overflow>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HideMiddlewareData {
    pub reference_hidden: bool,
    pub escaped: bool,
    pub reference_hidden_offset: SideObject,
    pub escaped_offsets: SideObject,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MiddlewareData {
    pub name: String,

    pub arrow: Option<ArrowMiddlewareData>,
    pub auto_placement: Option<AutoPlacementMiddlewareData>,
    pub flip: Option<FlipMiddlewareData>,
    pub hide: Option<HideMiddlewareData>,

    pub offset: Option<Coords>,
    pub shift: Option<Coords>,
}

#[derive(Clone)]
pub struct ComputePositionConfig {
    pub platform: Rc<dyn Platform>,
    pub placement: Option<Placement>,
    pub strategy: Option<Strategy>,
    pub middleware: Vec<Rc<dyn Middleware>>,
}

impl fmt::Debug for ComputePositionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComputePositionConfig")
            .field("placement", &self.placement)
            .field("strategy", &self.strategy)
            .finish()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ComputePositionReturn {
    pub coords: Coords,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MiddlewareReturn {
    pub data: BTreeMap<String, String>,
    pub reset: bool,
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
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

// TODO(Shaohua):
pub type ReferenceElement = String;
pub type FloatingElement = String;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Elements {
    pub reference: ReferenceElement,
    pub floating: FloatingElement,
}

#[derive(Clone)]
pub struct MiddlewareState {
    pub coords: Coords,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: MiddlewareData,
    pub elements: Elements,
    pub rects: ElementRects,
    pub platform: Rc<dyn Platform>,
}

impl fmt::Debug for MiddlewareState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddlewareState")
            .field("coords", &self.coords)
            .field("initial_placement", &self.initial_placement)
            .field("placement", &self.placement)
            .field("strategy", &self.strategy)
            .field("middleware_data", &self.middleware_data)
            .field("elements", &self.elements)
            .field("rects", &self.rects)
            .finish()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClientRectObject {
    pub rect: Rect,
    pub side: SideObject,
}

impl From<Rect> for ClientRectObject {
    fn from(rect: Rect) -> Self {
        let side = SideObject {
            left: rect.x,
            top: rect.y,
            right: rect.x + rect.width,
            bottom: rect.y + rect.height,
        };
        Self { rect, side }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PartialSideObject {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
}

impl From<PartialSideObject> for SideObject {
    fn from(val: PartialSideObject) -> Self {
        Self {
            top: val.top.unwrap_or_default(),
            right: val.right.unwrap_or_default(),
            bottom: val.bottom.unwrap_or_default(),
            left: val.left.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Padding {
    Number(f64),
    Side(PartialSideObject),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RootBoundary {
    Viewport,
    Document,
    Rect(Rect),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementContext {
    Reference,
    Floating,
}
