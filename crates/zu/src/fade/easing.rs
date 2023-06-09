// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::transition_duration::Easing;

#[must_use]
pub fn default_easing() -> Easing {
    Easing {
        enter: "var(--zu-easing-easeOut)".to_owned(),
        exit: "var(--zu-easing-easeIn)".to_owned(),
    }
}
