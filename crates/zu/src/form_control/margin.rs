// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::margin::Margin;

#[must_use]
pub const fn css_class(margin: Margin) -> &'static str {
    match margin {
        Margin::Dense => "ZuFormControl-marginDense",
        Margin::Normal => "ZuFormControl-marginNormal",
        Margin::None => "",
    }
}
