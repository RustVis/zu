// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::orientation::Orientation;

#[must_use]
pub const fn css_class(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Vertical => "ZuDivider-withChildrenVertical",
        Orientation::Horizontal => "ZuDivider-withChildrenHorizontal",
    }
}
