// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Changes the placement of the floating element to keep it in view.
//!
//! This prevents the floating element from overflowing along its side axis
//! by flipping it to the opposite side by default.

use crate::middleware::{Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState};
use crate::types::Overflow;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FlipMiddlewareData {
    pub index: Option<usize>,
    pub overflows: Vec<Overflow>,
}

impl FlipMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Flip::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

#[derive(Debug, Clone)]
pub struct Flip {
    pub option: (),
}

impl Flip {
    pub const NAME: &'static str = "flip";
}

impl Middleware for Flip {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
        todo!()
    }
}
