// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::types::{Boundary, ElementContext, MiddlewareState, Padding, RootBoundary, SideObject};

#[derive(Debug, Clone, PartialEq)]
pub struct DetectOverflowOption {
    pub boundary: Boundary,
    pub root_boundary: RootBoundary,
    pub element_context: ElementContext,
    pub alt_boundary: bool,
    pub padding: Padding,
}

impl Default for DetectOverflowOption {
    fn default() -> Self {
        Self {
            boundary: Boundary::ClippingAncestors,
            root_boundary: RootBoundary::Viewport,
            element_context: ElementContext::Floating,
            alt_boundary: false,
            padding: Padding::default(),
        }
    }
}

#[must_use]
pub fn detect_overflow(_state: &MiddlewareState, _option: &DetectOverflowOption) -> SideObject {
    unimplemented!()
}
