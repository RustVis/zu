// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::convert::Into;

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
    pub const fn get_list(self, is_start: bool, rtl: bool) -> [Self; 2] {
        match self {
            Self::Top | Self::Bottom => {
                let is_start = if rtl { !is_start } else { is_start };
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
    /// Alias of `side_axis()`.
    pub const fn main_axis(self) -> Axis {
        self.side_axis()
    }

    #[must_use]
    #[inline]
    pub const fn alignment_axis(self) -> Axis {
        self.side_axis().opposite()
    }

    #[must_use]
    #[inline]
    /// Get opposite placement.
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
        rtl: bool,
    ) -> Vec<Self> {
        let is_start = direction == Some(Alignment::Start);
        let side_list = self.side().get_list(is_start, rtl);
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
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
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

pub trait LengthTrait {
    fn length(&self, length: Length) -> f64;
    fn set_length(&mut self, length: Length, val: f64);
}

pub trait AxisTrait {
    fn axis(&self, axis: Axis) -> f64;
    fn set_axis(&mut self, axis: Axis, val: f64);
}

pub trait SideTrait {
    fn side(&self, side: Side) -> f64;
    fn set_side(&mut self, side: Side, val: f64);
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
pub struct Overflow {
    pub placement: Placement,
    pub overflows: Vec<f64>,
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
pub struct ClientRectObject {
    pub rect: Rect,
    pub side: SideObject,
}

impl From<Rect> for ClientRectObject {
    fn from(rect: Rect) -> Self {
        let side = rect.side_object();
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

#[derive(Debug, Clone, PartialEq)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

impl Default for Scale {
    #[must_use]
    #[inline]
    fn default() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct OffsetParent {
    pub client_width: f64,
    pub client_height: f64,
    pub client_left: f64,
    pub client_top: f64,
}
