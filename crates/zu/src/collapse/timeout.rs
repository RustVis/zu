// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timeout {
    Auto,
    Standard,
    Num(i32),
    Complex(ComplexTimeout),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexTimeout {
    pub appear: i32,
    pub enter: i32,
    pub exit: i32,
}

impl Default for Timeout {
    fn default() -> Self {
        Self::Standard
    }
}
