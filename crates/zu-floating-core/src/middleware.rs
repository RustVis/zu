// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::types::{
    ClientRectObject, Coords, Dimensions, LengthTrait, Overflow, PartialCoords, Placement, Rect,
    Scale, SideObject, Strategy,
};

pub trait Element: fmt::Debug + LengthTrait {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ElementContext {
    #[default]
    Floating,
    Reference,
}

impl ElementContext {
    #[must_use]
    #[inline]
    pub const fn alter(self) -> Self {
        match self {
            Self::Floating => Self::Reference,
            Self::Reference => Self::Floating,
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

#[derive(Debug, Default, Clone)]
pub enum Boundary {
    #[default]
    ClippingAncestors,
    Element(Rc<dyn Element>),
    Elements(Vec<Rc<dyn Element>>),
    Rect(Rect),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum RootBoundary {
    #[default]
    Viewport,
    Document,
    Rect(Rect),
}

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

/// Impl Platform trait to support new platform environment.
pub trait Platform: fmt::Debug {
    fn dimensions(&self, element: &Rc<dyn Element>) -> Dimensions;

    fn offset_parent(&self, element: &Rc<dyn Element>) -> Option<Rc<dyn Element>>;

    /// Returns true if layout direction is Right-To-Left.
    fn is_rtl(&self, element: &Rc<dyn Element>) -> bool;

    fn clipping_rect(
        &self,
        element: &Rc<dyn Element>,
        boundary: &Boundary,
        root_boundary: &RootBoundary,
        strategy: Strategy,
    ) -> Rect;

    fn element_rects(
        &self,
        reference_element: &Rc<dyn Element>,
        floating_element: &Rc<dyn Element>,
        strategy: Strategy,
    ) -> ElementRects;

    fn scale(&self, element: &Rc<dyn Element>) -> Scale;

    /// Convert (offset of parent)-relative-rect to viewport-relative-rect
    fn convert_relative_rect(
        &self,
        rect: &Rect,
        offset_parent: &Rc<dyn Element>,
        strategy: Strategy,
    ) -> Rect;
}

pub trait Middleware: fmt::Debug {
    // TODO(Shaohua): Add fn name()
    // TODO(Shaohua): Remove kind().
    fn kind(&self) -> MiddlewareDataKind;
    fn run(&self, state: &mut MiddlewareState) -> MiddlewareReturn;
}

pub type ComputePosition = fn(
    reference_element: &Rc<dyn Element>,
    floating_element: &Rc<dyn Element>,
    config: &ComputePositionConfig,
) -> ComputePositionReturn;

// TODO(Shaohua): Remove duplicates
pub type ComputePosition2 = fn(config: &ComputePositionConfig) -> ComputePositionReturn;

/// Custom positioning reference element.
pub trait VirtualElement {
    fn get_bounding_client_rect(&self) -> ClientRectObject;

    // TODO(Shaohua): Returns any type.
    fn get_context_element(&self);
}
