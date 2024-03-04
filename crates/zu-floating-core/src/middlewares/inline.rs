// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Improves positioning for inline reference elements that span over multiple lines.

use crate::middleware::{Middleware, MiddlewareDataKind, MiddlewareReturn, MiddlewareState};

#[derive(Debug, Clone)]
pub struct Inline {
    pub option: (),
}

impl Inline {
    pub const NAME: &'static str = "inline";
}

impl Middleware for Inline {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn kind(&self) -> MiddlewareDataKind {
        MiddlewareDataKind::Inline
    }

    fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
        todo!()
    }
}
