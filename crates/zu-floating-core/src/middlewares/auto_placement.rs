// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Chooses the placement that has the most space available automatically.

use crate::middleware::{Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState};
use crate::types::Overflow;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AutoPlacementMiddlewareData {
    pub index: Option<usize>,
    pub overflows: Vec<Overflow>,
}

impl AutoPlacementMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(AutoPlacement::NAME)
            .map(|boxed| boxed.downcast_ref())?
    }
}

#[derive(Debug, Clone)]
pub struct AutoPlacement {
    pub option: (),
}

impl AutoPlacement {
    pub const NAME: &'static str = "auto-placement";
}

impl Middleware for AutoPlacement {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
        todo!()
    }
}
