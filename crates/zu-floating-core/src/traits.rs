// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::types::{
    Axis, Boundary, ClientRectObject, ComputePositionConfig, ComputePositionReturn, Dimensions,
    ElementRects, Length, MiddlewareDataKind, MiddlewareReturn, MiddlewareState, Rect,
    RootBoundary, Scale, Side, Strategy,
};

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

pub trait Element: fmt::Debug + LengthTrait {}

/// Impl Platform trait to support new platform environment.
pub trait Platform: fmt::Debug {
    fn dimensions(&self, element: &Rc<dyn Element>) -> Dimensions;

    fn offset_parent(&self, element: &Rc<dyn Element>) -> Option<Rc<dyn Element>>;

    /// Returns true if layout direction is Right-To-Left.
    fn is_rtl(&self, element: &Rc<dyn Element>) -> bool;

    fn clipping_rect(
        &self,
        element: &Rc<dyn Element>,
        boundary: Boundary,
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
    fn kind(&self) -> MiddlewareDataKind;
    fn run(&self, state: &mut MiddlewareState) -> MiddlewareReturn;
}

pub type ComputePosition = fn(config: &ComputePositionConfig) -> ComputePositionReturn;

/// Custom positioning reference element.
pub trait VirtualElement {
    fn get_bounding_client_rect(&self) -> ClientRectObject;

    // TODO(Shaohua): Returns any type.
    fn get_context_element(&self);
}
