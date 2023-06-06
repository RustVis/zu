// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionDuration {
    Auto,
    Standard,
    Num(i32),
    Complex(ComplexTransitionDuration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexTransitionDuration {
    pub appear: i32,
    pub enter: i32,
    pub exit: i32,
}

impl Default for TransitionDuration {
    fn default() -> Self {
        Self::Standard
    }
}
