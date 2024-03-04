// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

use crate::platform::{ElementRects, Elements, Platform};
use crate::types::{Coords, PartialCoords, Placement, Strategy};

pub trait MiddlewareDataValue: fmt::Debug {}

pub struct MiddlewareData(BTreeMap<&'static str, Box<dyn Any>>);

impl Default for MiddlewareData {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MiddlewareData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddlewareData")
            .field("keys", &self.0.keys())
            .finish()
    }
}

impl MiddlewareData {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    #[inline]
    pub fn with_value(name: &'static str, value: Box<dyn Any>) -> Self {
        let mut this = Self(BTreeMap::new());
        this.0.insert(name, value);
        this
    }

    #[must_use]
    #[inline]
    pub fn get(&self, name: &'static str) -> Option<&Box<dyn Any>> {
        self.0.get(name)
    }

    #[inline]
    pub fn insert(&mut self, name: &'static str, value: Box<dyn Any>) {
        self.0.insert(name, value);
    }

    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }
}

#[derive(Debug, Default, Clone)]
pub enum MiddlewareResetRects {
    #[default]
    Nil,
    FromPlatform,
    Value(ElementRects),
}

#[derive(Debug, Default, Clone)]
pub struct MiddlewareReset {
    pub is_on: bool,
    pub rects: MiddlewareResetRects,
    pub placement: Option<Placement>,
}

#[derive(Debug, Default)]
pub struct MiddlewareReturn {
    pub coords: PartialCoords,
    pub data: MiddlewareData,
    pub reset: MiddlewareReset,
}

impl MiddlewareReturn {
    #[must_use]
    #[inline]
    pub fn from_data(data: MiddlewareData) -> Self {
        Self {
            data,
            ..Self::default()
        }
    }
}

pub struct MiddlewareState<'a> {
    pub coords: &'a Coords,
    pub initial_placement: Placement,
    pub placement: Placement,
    pub strategy: Strategy,
    pub middleware_data: &'a MiddlewareData,
    pub elements: &'a Elements,
    pub rects: &'a ElementRects,
    pub platform: &'a Rc<dyn Platform>,
}

impl<'a> fmt::Debug for MiddlewareState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiddlewareState")
            .field("coords", &self.coords)
            .field("initial_placement", &self.initial_placement)
            .field("placement", &self.placement)
            .field("strategy", &self.strategy)
            .field("middleware_data", self.middleware_data)
            .field("elements", self.elements)
            .field("rects", self.rects)
            .field("platform", self.platform)
            .finish()
    }
}

pub trait Middleware: fmt::Debug {
    fn name(&self) -> &str;

    fn run(&self, state: &MiddlewareState) -> MiddlewareReturn;
}
