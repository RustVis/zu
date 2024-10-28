// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::label_variant::LabelVariant;
use crate::styles::CssClass;

impl CssClass for LabelVariant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Filled => "ZuAlert-filled",
            Self::Outlined => "ZuAlert-outlined",
            Self::Standard => "ZuAlert-standard",
        }
    }
}
