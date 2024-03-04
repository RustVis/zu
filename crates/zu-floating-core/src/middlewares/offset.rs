// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by  Lesser General Public License that can be found
// in the LICENSE file.

//! Translates the floating element along the specified axes.
//!
//! This lets you add distance (margin or spacing) between the reference and floating element,
//! slightly alter the placement, or even create custom placements.

use crate::middleware::{
    Middleware, MiddlewareData, MiddlewareDataKind, MiddlewareReturn, MiddlewareState,
};
use crate::types::Coords;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct OffsetMiddlewareData {
    pub coords: Coords,
}

impl OffsetMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Offset::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

#[derive(Debug, Clone)]
pub struct Offset {
    pub option: (),
}

impl Offset {
    pub const NAME: &'static str = "offset";
}

impl Middleware for Offset {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn kind(&self) -> MiddlewareDataKind {
        MiddlewareDataKind::Offset
    }

    fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
        todo!()
    }
}
