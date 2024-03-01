// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

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

// TODO(Shaohua): Remove this enum.
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

// TODO(Shaohua): Replace MiddlewareData with BTreeMap.
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MiddlewareReset {
    pub reset: bool,
    pub rects: Option<ElementRects>,
    pub placement: Option<Placement>,
}

#[derive(Debug, Default, Clone, PartialEq)]
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
    // TODO(Shaohua): Add fn name()
    // TODO(Shaohua): Remove kind().
    fn kind(&self) -> MiddlewareDataKind;
    fn run(&self, state: &mut MiddlewareState) -> MiddlewareReturn;
}
