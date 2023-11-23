// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod create_svg_icon;

pub use create_svg_icon::create_svg_icon;

#[inline]
#[must_use]
pub fn input_value_to_bool(value: &str) -> bool {
    value == "on"
}
