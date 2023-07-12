// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::anchor_origin::{AnchorOrigin, Horizontal, Vertical};

pub const fn css_class(origin: AnchorOrigin) -> &'static str {
    match origin {
        AnchorOrigin {
            horizontal: Horizontal::Left,
            vertical: Vertical::Top,
        } => "ZuBadge-anchorOriginTopLeft",
        AnchorOrigin {
            horizontal: Horizontal::Right,
            vertical: Vertical::Top,
        } => "ZuBadge-anchorOriginTopRight",
        AnchorOrigin {
            horizontal: Horizontal::Left,
            vertical: Vertical::Bottom,
        } => "ZuBadge-anchorOriginBottomLeft",
        AnchorOrigin {
            horizontal: Horizontal::Right,
            vertical: Vertical::Bottom,
        } => "ZuBadge-anchorOriginBottomRight",
    }
}
