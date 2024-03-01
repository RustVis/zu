// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::middleware::{Middleware, MiddlewareData};
use crate::types::{Coords, Dimensions, LengthTrait, Placement, Rect, Scale, Strategy};

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

// TODO(Shaohua): Remove duplicates
pub type ComputePosition2 = fn(config: &ComputePositionConfig) -> ComputePositionReturn;
