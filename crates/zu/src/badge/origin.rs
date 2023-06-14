// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::origin::{HorizontalOrigin, Origin, VerticalOrigin};

pub fn css_class(origin: Origin) -> &'static str {
    match origin {
        Origin {
            horizontal: HorizontalOrigin::Left,
            vertical: VerticalOrigin::Top,
        } => "ZuBadge-anchorOriginTopLeft",
        Origin {
            horizontal: HorizontalOrigin::Right,
            vertical: VerticalOrigin::Top,
        } => "ZuBadge-anchorOriginTopRight",
        Origin {
            horizontal: HorizontalOrigin::Left,
            vertical: VerticalOrigin::Bottom,
        } => "ZuBadge-anchorOriginBottomLeft",
        Origin {
            horizontal: HorizontalOrigin::Right,
            vertical: VerticalOrigin::Bottom,
        } => "ZuBadge-anchorOriginBottomRight",
        _ => {
            log::warn!("Invalid anchor origin value is used, {origin:?}");
            ""
        }
    }
}
