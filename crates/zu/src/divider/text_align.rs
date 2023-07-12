// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::text_align::TextAlign;

#[must_use]
pub const fn css_class(text_align: TextAlign) -> &'static str {
    match text_align {
        TextAlign::Left => "ZuDivider-textAlignLeft",
        TextAlign::Right => "ZuDivider-textAlignRight",
        _ => "",
    }
}
