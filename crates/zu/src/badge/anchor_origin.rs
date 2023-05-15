// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::side::Side;
use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnchorOrigin {
    horizontal: Side,
    vertical: Side,
}

impl Default for AnchorOrigin {
    fn default() -> Self {
        Self {
            horizontal: Side::Top,
            vertical: Side::Right,
        }
    }
}

impl CssClass for AnchorOrigin {
    fn css_class(&self) -> &'static str {
        match self {
            Self {
                horizontal: Side::Left,
                vertical: Side::Top,
            } => "ZuBadge-anchorOriginTopLeft",
            Self {
                horizontal: Side::Right,
                vertical: Side::Top,
            } => "ZuBadge-anchorOriginTopRight",
            Self {
                horizontal: Side::Left,
                vertical: Side::Bottom,
            } => "ZuBadge-anchorOriginBottomLeft",
            Self {
                horizontal: Side::Right,
                vertical: Side::Bottom,
            } => "ZuBadge-anchorOriginBottomRight",
            _ => {
                log::warn!("Invalid anchor origin value is used, {self:?}");
                ""
            }
        }
    }
}
