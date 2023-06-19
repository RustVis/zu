// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Column {
    Auto,
    Bool(bool),
    Num(i32),
}

impl Default for Column {
    fn default() -> Self {
        Self::Bool(false)
    }
}
