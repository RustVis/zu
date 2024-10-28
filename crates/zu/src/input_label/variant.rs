// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::label_variant::LabelVariant;

#[must_use]
pub const fn css_class(variant: LabelVariant) -> &'static str {
    match variant {
        LabelVariant::Filled => "ZuInputLabel-filled",
        LabelVariant::Outlined => "ZuInputLabel-outlined",
        LabelVariant::Standard => "ZuInputLabel-standard",
    }
}
