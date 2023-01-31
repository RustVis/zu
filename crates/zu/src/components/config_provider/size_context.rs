// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Node size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeType {
    Small,
    Middle,
    Large,
}

impl SizeType {
    /// Convert to class suffix.
    #[must_use]
    pub const fn to_class(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Middle => "md",
            Self::Large => "lg",
        }
    }
}
