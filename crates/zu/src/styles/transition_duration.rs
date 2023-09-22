// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionDuration {
    Auto,
    Num(i32),
    Complex(ComplexTransitionDuration),
}

impl TransitionDuration {
    #[must_use]
    pub fn standard() -> Self {
        Self::Complex(ComplexTransitionDuration {
            appear: String::new(),
            enter: "var(--zu-duration-standard)".to_owned(),
            exit: "var(--zu-duration-standard)".to_owned(),
        })
    }

    #[must_use]
    pub fn enter_leave_screen() -> Self {
        Self::Complex(ComplexTransitionDuration {
            appear: String::new(),
            enter: "var(--zu-duration-enteringScreen)".to_owned(),
            exit: "var(--zu-duration-leavingScreen)".to_owned(),
        })
    }
}

impl Default for TransitionDuration {
    fn default() -> Self {
        Self::enter_leave_screen()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexTransitionDuration {
    pub appear: String,
    pub enter: String,
    pub exit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Easing {
    pub enter: String,
    pub exit: String,
}
