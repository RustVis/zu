// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Contained,
    Outlined,
    Text,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Text
    }
}

impl ButtonVariant {
    #[must_use]
    pub const fn capitalize(self) -> &'static str {
        match self {
            Self::Contained => "Contained",
            Self::Outlined => "Outlined",
            Self::Text => "Text",
        }
    }
}
