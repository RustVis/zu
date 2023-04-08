// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::detect_overflow::{detect_overflow, DetectOverflowOption};
use crate::traits::Middleware;
use crate::types::{
    HideMiddlewareData, MiddlewareData, MiddlewareDataKind, MiddlewareReturn, MiddlewareState,
    SideObject,
};

#[derive(Debug, Clone)]
pub struct HideOption {
    pub strategy: HideStrategy,
    pub detect_overflow: DetectOverflowOption,
}

/// The strategy used to determine when to hide the floating element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HideStrategy {
    ReferenceHidden,
    Escaped,
}

#[derive(Debug, Clone)]
pub struct Hide {
    pub option: HideOption,
}

impl Middleware for Hide {
    fn kind(&self) -> MiddlewareDataKind {
        MiddlewareDataKind::Hide
    }

    fn run(&self, state: &MiddlewareState) -> MiddlewareReturn {
        let rects = &state.rects;
        let hide_data = match self.option.strategy {
            HideStrategy::ReferenceHidden => {
                let overflow: SideObject = detect_overflow(state, &self.option.detect_overflow);
                let offsets: SideObject = overflow.offset(&rects.reference);

                HideMiddlewareData {
                    reference_hidden: Some(offsets.is_any_side_fully_clipped()),
                    reference_hidden_offset: Some(offsets),
                    ..Default::default()
                }
            }
            HideStrategy::Escaped => {
                let overflow = detect_overflow(state, &self.option.detect_overflow);
                let offsets = overflow.offset(&rects.floating);

                HideMiddlewareData {
                    escaped: Some(offsets.is_any_side_fully_clipped()),
                    escaped_offsets: Some(offsets),
                    ..Default::default()
                }
            }
        };

        let data = MiddlewareData {
            kind: self.kind(),
            hide: Some(hide_data),
            ..Default::default()
        };
        MiddlewareReturn {
            data,
            ..Default::default()
        }
    }
}
