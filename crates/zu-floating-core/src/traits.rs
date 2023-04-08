// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::types::{
    Axis, ComputePositionConfig, ComputePositionReturn, Dimensions, Length, MiddlewareReturn,
    MiddlewareState, Side,
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
pub trait Platform {
    fn dimensions(&self) -> Dimensions;

    fn offset_parent(&self, element: &Rc<dyn Element>) -> Rc<dyn Element>;

    /// Returns true if layout direction is Right-To-Left.
    fn is_rtl(&self) -> bool;

    fn clipping_rect(&mut self) -> (f32, f32);
}

pub trait Middleware {
    fn name(&self) -> &str;
    fn run(&mut self, state: &MiddlewareState) -> MiddlewareReturn;
}

pub type ComputePosition = fn(config: &ComputePositionConfig) -> ComputePositionReturn;
