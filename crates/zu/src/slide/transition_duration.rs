// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::transition_duration::{ComplexTransitionDuration, TransitionDuration};

#[must_use]
pub fn default_duration() -> TransitionDuration {
    TransitionDuration::Complex(ComplexTransitionDuration {
        appear: String::new(),
        enter: "var(--zu-duration-enteringScreen)".to_owned(),
        exit: "var(--zu-duration-leavingScreen)".to_owned(),
    })
}
