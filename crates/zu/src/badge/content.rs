// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Used as badge content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    Str(String),
    Num(i32),
}

impl Default for Content {
    fn default() -> Self {
        Self::Num(0)
    }
}
