// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Vertical,
    #[default]
    Horizontal,
}

impl Orientation {
    #[must_use]
    pub const fn capitalize(self) -> &'static str {
        match self {
            Self::Vertical => "Vertical",
            Self::Horizontal => "Horizontal",
        }
    }
}
