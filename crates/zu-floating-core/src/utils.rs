// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

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

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Placement {
    TopStart,
    #[default]
    TopCenter,
    TopEnd,
    RightStart,
    RightCenter,
    RightEnd,
    BottomStart,
    BottomCenter,
    BottomEnd,
    LeftStart,
    LeftCenter,
    LeftEnd,
}

impl Placement {
    #[must_use]
    #[inline]
    pub const fn side(self) -> Side {
        match self {
            Self::TopStart | Self::TopCenter | Self::TopEnd => Side::Top,
            Self::RightStart | Self::RightCenter | Self::RightEnd => Side::Right,
            Self::BottomStart | Self::BottomCenter | Self::BottomEnd => Side::Bottom,
            Self::LeftStart | Self::LeftCenter | Self::LeftEnd => Side::Left,
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
    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }

    #[must_use]
    #[inline]
    pub const fn length(self) -> Length {
        match self {
            Self::X => Length::Width,
            Self::Y => Length::Height,
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
    pub const fn opposite(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

impl Coords {
    #[must_use]
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self { x: value, y: value }
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
        axis.length()
    }
}

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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rect {
    pub coords: Coords,
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Padding {
    Num(f64),
    Partial(SideObject),
}

impl Default for Padding {
    fn default() -> Self {
        Self::Num(0.0)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClientRectObject {
    pub rect: Rect,
    pub side_object: SideObject,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

/// Custom positioning reference element.
pub trait VirtualElement {
    fn get_bounding_client_rect(&self) -> ClientRectObject;

    // TODO(Shaohua): Returns any type.
    fn get_context_element(&self);
}
