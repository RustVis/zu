// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

use std::convert::Into;
use std::fmt;
use std::rc::Rc;

use crate::traits::{AxisTrait, Element, LengthTrait, Middleware, Platform, SideTrait};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Alignment {
    #[default]
    Start,
    End,
}

impl Alignment {
    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Start => Self::End,
            Self::End => Self::Start,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Side {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
        }
    }

    #[must_use]
    #[inline]
    pub const fn axis(self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Y,
            Self::Top | Self::Bottom => Axis::X,
        }
    }

    #[must_use]
    #[inline]
    pub const fn get_list(self, is_start: bool, is_rtl: bool) -> [Self; 2] {
        match self {
            Self::Top | Self::Bottom => {
                let is_start = if is_rtl { !is_start } else { is_start };
                if is_start {
                    [Self::Left, Self::Right]
                } else {
                    [Self::Right, Self::Left]
                }
            }
            Self::Left | Self::Right => {
                if is_start {
                    [Self::Top, Self::Bottom]
                } else {
                    [Self::Bottom, Self::Top]
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum AlignedPlacement {
    #[default]
    TopStart,
    TopEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
    LeftStart,
    LeftEnd,
}

impl AlignedPlacement {
    #[must_use]
    #[inline]
    pub const fn alignment(self) -> Alignment {
        match self {
            Self::TopStart | Self::RightStart | Self::BottomStart | Self::LeftStart => {
                Alignment::Start
            }
            Self::TopEnd | Self::RightEnd | Self::BottomEnd | Self::LeftEnd => Alignment::End,
        }
    }

    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::TopStart => Self::BottomStart,
            Self::TopEnd => Self::BottomEnd,
            Self::RightStart => Self::LeftStart,
            Self::RightEnd => Self::LeftEnd,
            Self::BottomStart => Self::TopStart,
            Self::BottomEnd => Self::TopEnd,
            Self::LeftStart => Self::RightStart,
            Self::LeftEnd => Self::RightEnd,
        }
    }

    #[must_use]
    #[inline]
    pub const fn opposite_alignment(self) -> Self {
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

impl From<AlignedPlacement> for Alignment {
    fn from(align_placement: AlignedPlacement) -> Self {
        align_placement.alignment()
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Placement {
    #[default]
    TopStart,
    Top,
    TopEnd,
    RightStart,
    Right,
    RightEnd,
    BottomStart,
    Bottom,
    BottomEnd,
    LeftStart,
    Left,
    LeftEnd,
}

impl Placement {
    #[must_use]
    #[inline]
    pub const fn new(side: Side, alignment: Alignment) -> Self {
        match (side, alignment) {
            (Side::Top, Alignment::Start) => Self::TopStart,
            (Side::Top, Alignment::End) => Self::TopEnd,
            (Side::Right, Alignment::Start) => Self::RightStart,
            (Side::Right, Alignment::End) => Self::RightEnd,
            (Side::Bottom, Alignment::Start) => Self::BottomStart,
            (Side::Bottom, Alignment::End) => Self::BottomEnd,
            (Side::Left, Alignment::Start) => Self::LeftStart,
            (Side::Left, Alignment::End) => Self::LeftEnd,
        }
    }

    #[must_use]
    #[inline]
    pub const fn side(self) -> Side {
        match self {
            Self::TopStart | Self::Top | Self::TopEnd => Side::Top,
            Self::RightStart | Self::Right | Self::RightEnd => Side::Right,
            Self::BottomStart | Self::Bottom | Self::BottomEnd => Side::Bottom,
            Self::LeftStart | Self::Left | Self::LeftEnd => Side::Left,
        }
    }

    #[must_use]
    #[inline]
    pub const fn alignment(self) -> Option<Alignment> {
        match self {
            Self::TopStart | Self::RightStart | Self::BottomStart | Self::LeftStart => {
                Some(Alignment::Start)
            }
            Self::TopEnd | Self::RightEnd | Self::BottomEnd | Self::LeftEnd => Some(Alignment::End),
            _ => None,
        }
    }

    #[must_use]
    #[inline]
    pub const fn side_axis(self) -> Axis {
        self.side().axis()
    }

    #[must_use]
    #[inline]
    pub const fn alignment_axis(self) -> Axis {
        self.side_axis().opposite()
    }

    /// Get opposite placement.
    #[must_use]
    #[inline]
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

    /// Get opposite alignment placement.
    #[must_use]
    #[inline]
    pub const fn opposite_alignment(self) -> Self {
        match self {
            Self::TopStart => Self::TopEnd,
            Self::Top => Self::Top,
            Self::TopEnd => Self::TopStart,

            Self::RightStart => Self::RightEnd,
            Self::Right => Self::Right,
            Self::RightEnd => Self::RightStart,

            Self::BottomStart => Self::BottomEnd,
            Self::Bottom => Self::Bottom,
            Self::BottomEnd => Self::BottomStart,

            Self::LeftStart => Self::LeftEnd,
            Self::Left => Self::Left,
            Self::LeftEnd => Self::LeftStart,
        }
    }

    #[must_use]
    #[inline]
    pub const fn expand(self) -> [Self; 3] {
        let opposite_placement = self.opposite();
        [
            self.opposite_alignment(),
            opposite_placement,
            opposite_placement.opposite_alignment(),
        ]
    }

    #[must_use]
    #[inline]
    pub fn opposite_axis_placements(
        self,
        flip_alignment: bool,
        direction: Option<Alignment>,
        is_rtl: bool,
    ) -> Vec<Self> {
        let is_start = direction == Some(Alignment::Start);
        let side_list = self.side().get_list(is_start, is_rtl);
        self.alignment().map_or_else(
            || side_list.map(Into::into).to_vec(),
            |alignment| {
                let mut list: Vec<_> = side_list
                    .into_iter()
                    .map(|side| Self::new(side, alignment))
                    .collect();
                if flip_alignment {
                    let mut list2: Vec<_> = list
                        .iter()
                        .map(|place| place.opposite_alignment())
                        .collect();
                    list.append(&mut list2);
                }
                list
            },
        )
    }
}

impl From<Side> for Placement {
    #[must_use]
    #[inline]
    fn from(side: Side) -> Self {
        match side {
            Side::Top => Self::Top,
            Side::Right => Self::Right,
            Side::Bottom => Self::Bottom,
            Side::Left => Self::Left,
        }
    }
}

impl From<Placement> for Side {
    #[must_use]
    #[inline]
    fn from(placement: Placement) -> Self {
        placement.side()
    }
}

impl From<AlignedPlacement> for Placement {
    fn from(align_placement: AlignedPlacement) -> Self {
        match align_placement {
            AlignedPlacement::TopStart => Self::TopStart,
            AlignedPlacement::TopEnd => Self::TopEnd,

            AlignedPlacement::RightStart => Self::RightStart,
            AlignedPlacement::RightEnd => Self::RightEnd,

            AlignedPlacement::BottomStart => Self::BottomStart,
            AlignedPlacement::BottomEnd => Self::BottomEnd,

            AlignedPlacement::LeftStart => Self::LeftStart,
            AlignedPlacement::LeftEnd => Self::LeftEnd,
        }
    }
}

impl From<Placement> for Axis {
    #[must_use]
    #[inline]
    fn from(placement: Placement) -> Self {
        placement.side_axis()
    }
}

impl From<Placement> for Option<Alignment> {
    #[must_use]
    #[inline]
    fn from(placement: Placement) -> Self {
        placement.alignment()
    }
}

// impl Placement {
//     /// Returns main side and cross side.
//     #[must_use]
//     pub fn alignment_sides(self, rects: &ElementRects, rtl: bool) -> [Side; 2] {
//         let alignment = self.alignment();
//         let main_axis: Axis = self.main_axis();
//         let length: Length = main_axis.into();
//
//         let mut main_alignment_side: Side = match main_axis {
//             Axis::X => {
//                 let alignment_start = if rtl {
//                     Alignment::End
//                 } else {
//                     Alignment::Start
//                 };
//
//                 if alignment == Some(alignment_start) {
//                     Side::Right
//                 } else {
//                     Side::Left
//                 }
//             }
//             Axis::Y => {
//                 if alignment == Some(Alignment::Start) {
//                     Side::Bottom
//                 } else {
//                     Side::Top
//                 }
//             }
//         };
//
//         if rects.reference.length(length) > rects.floating.length(length) {
//             main_alignment_side = main_alignment_side.opposite();
//         }
//
//         let cross_side = main_alignment_side.opposite();
//         [main_alignment_side, cross_side]
//     }
// }

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Strategy {
    #[default]
    Absolute,
    Fixed,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Axis {
    #[default]
    X,
    Y,
}

impl Axis {
    /// Get cross axis
    #[must_use]
    #[inline]
    pub const fn cross(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }

    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

impl Coords {
    #[must_use]
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self { x: value, y: value }
    }

    #[must_use]
    #[inline]
    pub const fn opposite(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl AxisTrait for Coords {
    fn axis(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    fn set_axis(&mut self, axis: Axis, val: f64) {
        match axis {
            Axis::X => self.x = val,
            Axis::Y => self.y = val,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Length {
    #[default]
    Width,
    Height,
}

impl From<Axis> for Length {
    #[must_use]
    #[inline]
    fn from(axis: Axis) -> Self {
        match axis {
            Axis::X => Self::Width,
            Axis::Y => Self::Height,
        }
    }
}

/// Alias of Geometry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SideObject {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl SideTrait for SideObject {
    fn side(&self, side: Side) -> f64 {
        match side {
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
            Side::Left => self.left,
        }
    }

    fn set_side(&mut self, side: Side, val: f64) {
        match side {
            Side::Top => self.top = val,
            Side::Right => self.right = val,
            Side::Bottom => self.bottom = val,
            Side::Left => self.left = val,
        }
    }
}

impl SideObject {
    #[must_use]
    #[inline]
    pub const fn new(num: f64) -> Self {
        Self {
            top: num,
            right: num,
            bottom: num,
            left: num,
        }
    }

    #[must_use]
    pub fn offset(&self, rect: &Rect) -> Self {
        Self {
            top: self.top - rect.height,
            right: self.right - rect.width,
            bottom: self.bottom - rect.height,
            left: self.left - rect.width,
        }
    }

    #[must_use]
    pub fn is_any_side_fully_clipped(&self) -> bool {
        self.top >= 0.0 || self.right >= 0.0 || self.bottom >= 0.0 || self.left >= 0.0
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PartialCoords {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl PartialCoords {
    #[must_use]
    pub const fn new(axis: Axis, val: f64) -> Self {
        match axis {
            Axis::X => Self {
                x: Some(val),
                y: None,
            },
            Axis::Y => Self {
                x: None,
                y: Some(val),
            },
        }
    }

    pub fn set(&mut self, axis: Axis, val: f64) {
        match axis {
            Axis::X => self.x = Some(val),
            Axis::Y => self.y = Some(val),
        }
    }
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
    pub reference_hidden: Option<bool>,
    pub reference_hidden_offset: Option<SideObject>,
    pub escaped: Option<bool>,
    pub escaped_offsets: Option<SideObject>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiddlewareDataKind {
    Nil,

    Arrow,
    AutoPlacement,
    Flip,
    Hide,

    Offset,
    Shift,
    Size,
}

impl Default for MiddlewareDataKind {
    fn default() -> Self {
        Self::Nil
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MiddlewareData {
    pub kind: MiddlewareDataKind,

    pub arrow: Option<ArrowMiddlewareData>,
    pub auto_placement: Option<AutoPlacementMiddlewareData>,
    pub flip: Option<FlipMiddlewareData>,
    pub hide: Option<HideMiddlewareData>,

    pub offset: Option<Coords>,
    pub shift: Option<Coords>,
    pub size: Option<bool>,
}

#[derive(Clone)]
pub struct ComputePositionConfig {
    pub platform: Rc<dyn Platform>,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middlewares: Vec<Rc<dyn Middleware>>,
}

impl fmt::Debug for ComputePositionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComputePositionConfig")
            .field("platform", &self.platform)
            .field("placement", &self.placement)
            .field("strategy", &self.strategy)
            .field("middlewares", &self.middlewares)
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

pub type ComputePosition = fn(
    reference_element: &Rc<dyn Element>,
    floating_element: &Rc<dyn Element>,
    config: &ComputePositionConfig,
) -> ComputePositionReturn;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MiddlewareReset {
    pub flag: Option<bool>,
    pub rects: Option<ElementRects>,
    pub placement: Option<Placement>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MiddlewareReturn {
    pub coords: PartialCoords,
    pub data: MiddlewareData,
    pub reset: MiddlewareReset,
}

impl LengthTrait for Dimensions {
    fn length(&self, length: Length) -> f64 {
        match length {
            Length::Width => self.width,
            Length::Height => self.height,
        }
    }

    fn set_length(&mut self, length: Length, val: f64) {
        match length {
            Length::Width => self.width = val,
            Length::Height => self.height = val,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    #[must_use]
    #[inline]
    pub const fn coords(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y,
        }
    }

    #[must_use]
    #[inline]
    pub const fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
        }
    }

    #[must_use]
    #[inline]
    pub fn side_object(&self) -> SideObject {
        SideObject {
            top: self.y,
            right: self.x + self.width,
            bottom: self.y + self.height,
            left: self.x,
        }
    }
}

impl LengthTrait for Rect {
    fn length(&self, length: Length) -> f64 {
        match length {
            Length::Width => self.width,
            Length::Height => self.height,
        }
    }

    fn set_length(&mut self, length: Length, val: f64) {
        match length {
            Length::Width => self.width = val,
            Length::Height => self.height = val,
        }
    }
}

impl AxisTrait for Rect {
    fn axis(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    fn set_axis(&mut self, axis: Axis, val: f64) {
        match axis {
            Axis::X => self.x = val,
            Axis::Y => self.y = val,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

#[derive(Debug, Clone)]
pub struct Elements {
    pub reference: Rc<dyn Element>,
    pub floating: Rc<dyn Element>,
}

impl Elements {
    #[must_use]
    pub fn element(&self, element_context: ElementContext) -> Rc<dyn Element> {
        match element_context {
            ElementContext::Reference => self.reference.clone(),
            ElementContext::Floating => self.floating.clone(),
        }
    }
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
            .field("platform", &self.platform)
            .finish()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClientRectObject {
    pub rect: Rect,
    pub side_object: SideObject,
}

impl From<Rect> for ClientRectObject {
    fn from(rect: Rect) -> Self {
        let side_object = rect.side_object();
        Self { rect, side_object }
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
    #[must_use]
    #[inline]
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

impl Default for Padding {
    fn default() -> Self {
        Self::Number(0.0)
    }
}

impl From<Padding> for SideObject {
    fn from(padding: Padding) -> Self {
        match padding {
            Padding::Number(num) => Self::new(num),
            Padding::Side(side) => side.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary {
    ClippingAncestors,
}

impl Default for Boundary {
    fn default() -> Self {
        Self::ClippingAncestors
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RootBoundary {
    Viewport,
    Document,
    Rect(Rect),
}

impl Default for RootBoundary {
    fn default() -> Self {
        Self::Viewport
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementContext {
    Floating,
    Reference,
}

impl Default for ElementContext {
    fn default() -> Self {
        Self::Floating
    }
}

impl ElementContext {
    #[must_use]
    pub const fn alter(self) -> Self {
        match self {
            Self::Floating => Self::Reference,
            Self::Reference => Self::Floating,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}
