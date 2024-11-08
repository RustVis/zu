// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::any::Any;
use std::fmt;
use std::rc::Rc;

use crate::types::{ClientRect, Dimensions, LengthTrait, Rect, Scale, Strategy};

// TODO(Shaohua): Simplify Element trait.
pub trait Element: fmt::Debug + LengthTrait {
    /// This method is used to dynamic cast.
    fn as_any(&self) -> &dyn Any;
}

//  TODO(Shaohua): Remove Rc<> wrapper.
pub type ElementRef<'a> = &'a Rc<dyn Element>;

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
    Elements(Elements),
    Rect(Rect),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum RootBoundary {
    #[default]
    Viewport,
    Document,
    Rect(Rect),
}

/// `Platform` interface methods to work with the current platform.
///
/// Impl `Platform` trait to support new platform environment.
pub trait Platform: fmt::Debug {
    /// Takes in the elements and the positioning strategy and returns the element Rect objects.
    fn element_rects(
        &self,
        reference_element: &Rc<dyn Element>,
        floating_element: &Rc<dyn Element>,
        strategy: Strategy,
    ) -> ElementRects;

    /// Returns the dimensions of an element.
    fn dimensions(&self, element: &Rc<dyn Element>) -> Dimensions;

    /// Returns the Rect (relative to the viewport) whose outside bounds
    /// will clip the given element. For instance, the viewport itself.
    fn clipping_rect(
        &self,
        element: &Rc<dyn Element>,
        boundary: &Boundary,
        root_boundary: &RootBoundary,
        strategy: Strategy,
    ) -> Rect;

    /// Convert (offset of parent)-relative-rect to viewport-relative-rect.
    ///
    /// This method is optional.
    fn convert_relative_rect(
        &self,
        rect: &Rect,
        _offset_parent: &Rc<dyn Element>,
        _strategy: Strategy,
    ) -> Rect {
        rect.clone()
    }

    /// Returns the offsetParent of a given element.
    ///
    /// This method is optional.
    fn offset_parent(&self, _element: &Rc<dyn Element>) -> Option<Rc<dyn Element>> {
        None
    }

    /// Determines if the current value is an element.
    ///
    /// This method is optional.
    fn is_element(&self, _value: &Rc<dyn Element>) -> Option<bool> {
        None
    }

    /// Returns the document element.
    ///
    /// This method is optional.
    fn document_element(&self, _element: &Rc<dyn Element>) -> Option<Rc<dyn Element>> {
        None
    }

    /// Returns an array of `ClientRects`.
    ///
    /// This method is optional.
    fn client_rects(&self, _element: &Rc<dyn Element>) -> Option<Vec<ClientRect>> {
        None
    }

    /// Returns true if layout direction is Right-To-Left.
    ///
    /// This method is optional.
    fn is_rtl(&self, _element: &Rc<dyn Element>) -> bool {
        false
    }

    /// Get scale factor of document.
    ///
    /// This method is optional.
    fn scale(&self, _element: &Rc<dyn Element>) -> Scale {
        Scale::default()
    }
}
