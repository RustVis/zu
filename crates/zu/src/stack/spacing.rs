// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::spacing::Spacing;

#[must_use]
pub const fn css_cls(spacing: Spacing) -> &'static str {
    match spacing {
        Spacing::None => "ZuStack-spacing-none",
        Spacing::XXSmall => "ZuStack-spacing-xxs",
        Spacing::XSmall => "ZuStack-spacing-xs",
        Spacing::SmallNudge => "ZuStack-spacing-sNudge",
        Spacing::Small => "ZuStack-spacing-s",
        Spacing::MiddleNudge => "ZuStack-spacing-mNudge",
        Spacing::Middle => "ZuStack-spacing-m",
        Spacing::Large => "ZuStack-spacing-l",
        Spacing::XLarge => "ZuStack-spacing-xl",
        Spacing::XXLarge => "ZuStack-spacing-xxl",
        Spacing::XXXLarge => "ZuStack-spacing-xxxl",
    }
}
