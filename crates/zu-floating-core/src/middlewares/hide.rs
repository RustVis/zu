// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! A data provider that allows you to hide the floating element in applicable situations.
//!
//! This is useful for situations where you want to hide the floating element
//! because it appears detached from the reference element (or attached to nothing).

use crate::detect_overflow::{detect_overflow, DetectOverflowOption};
use crate::middleware::{
    Middleware, MiddlewareData, MiddlewareDataKind, MiddlewareReturn, MiddlewareState,
};
use crate::types::SideObject;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HideMiddlewareData {
    pub reference_hidden: Option<bool>,
    pub reference_hidden_offset: Option<SideObject>,
    pub escaped: Option<bool>,
    pub escaped_offsets: Option<SideObject>,
}

impl HideMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Hide::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

#[derive(Debug, Clone)]
pub struct HideOption {
    pub strategy: HideStrategy,
}

/// The strategy used to determine when to hide the floating element.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HideStrategy {
    #[default]
    ReferenceHidden,
    Escaped,
}

#[derive(Debug, Clone)]
pub struct Hide {
    pub option: HideOption,
    pub detect_overflow_option: DetectOverflowOption,
}

impl Hide {
    pub const NAME: &'static str = "hide";
}

impl Middleware for Hide {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn kind(&self) -> MiddlewareDataKind {
        MiddlewareDataKind::Hide
    }

    fn run(&self, state: &MiddlewareState) -> MiddlewareReturn {
        let rects = &state.rects;
        let hide_data = match self.option.strategy {
            HideStrategy::ReferenceHidden => {
                let overflow: SideObject = detect_overflow(state, &self.detect_overflow_option);
                let offsets: SideObject = overflow.offset(&rects.reference);

                HideMiddlewareData {
                    reference_hidden: Some(offsets.is_any_side_fully_clipped()),
                    reference_hidden_offset: Some(offsets),
                    ..Default::default()
                }
            }
            HideStrategy::Escaped => {
                let overflow = detect_overflow(state, &self.detect_overflow_option);
                let offsets = overflow.offset(&rects.floating);

                HideMiddlewareData {
                    escaped: Some(offsets.is_any_side_fully_clipped()),
                    escaped_offsets: Some(offsets),
                    ..Default::default()
                }
            }
        };

        let data = MiddlewareData::with_value(Self::NAME, Box::new(hide_data));
        MiddlewareReturn::from_data(data)
    }
}
