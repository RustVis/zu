// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Shifts the floating element to keep it in view.
//!
//! This prevents the floating element from overflowing along its axis of alignment,
//! thereby preserving the side it’s placed on.

use crate::middleware::{
    Middleware, MiddlewareData, MiddlewareDataKind, MiddlewareReturn, MiddlewareState,
};
use crate::types::Coords;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ShiftMiddlewareData {
    pub coords: Coords,
}

impl ShiftMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Shift::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

#[derive(Debug, Clone)]
pub struct Shift {
    pub option: (),
}

impl Shift {
    pub const NAME: &'static str = "shift";
}

impl Middleware for Shift {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn kind(&self) -> MiddlewareDataKind {
        MiddlewareDataKind::Shift
    }

    fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
        todo!()
    }
}
