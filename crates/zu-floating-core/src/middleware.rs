// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use crate::platform::{ElementRects, Elements, Platform};
use crate::types::{Coords, Overflow, PartialCoords, Placement, SideObject, Strategy};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArrowMiddlewareData {
    pub coords: PartialCoords,
    pub center_offset: f64,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MiddlewareDataKind {
    #[default]
    Nil,

    Arrow,
    AutoPlacement,
    Flip,
    Hide,

    Offset,
    Shift,
    Size,

    Custom,
}

pub type CustomMiddlewareData = BTreeMap<&'static str, Option<Rc<dyn Any>>>;

#[derive(Debug, Default, Clone)]
pub enum MiddlewareData {
    #[default]
    Nil,

    Arrow(ArrowMiddlewareData),
    AutoPlacement(AutoPlacementMiddlewareData),
    Flip(FlipMiddlewareData),
    Hide(HideMiddlewareData),

    Offset(Coords),
    Shift(Coords),
    Size(bool),

    Custom(CustomMiddlewareData),
}

impl MiddlewareData {
    #[must_use]
    #[inline]
    pub const fn shift(&self) -> Option<&Coords> {
        match self {
            Self::Shift(coords) => Some(coords),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MiddlewareReset {
    pub reset: bool,
    pub rects: Option<ElementRects>,
    pub placement: Option<Placement>,
}

#[derive(Debug, Default, Clone)]
pub struct MiddlewareReturn {
    pub coords: PartialCoords,
    pub data: MiddlewareData,
    pub reset: MiddlewareReset,
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

pub trait Middleware: fmt::Debug {
    fn name(&self) -> &'static str;

    fn kind(&self) -> MiddlewareDataKind;

    fn run(&self, state: &mut MiddlewareState) -> MiddlewareReturn;
}
