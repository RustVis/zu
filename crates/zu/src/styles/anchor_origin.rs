// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnchorOrigin {
    pub horizontal: Horizontal,
    pub vertical: Vertical,
}

impl AnchorOrigin {
    #[must_use]
    pub const fn top_left() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Top,
        }
    }

    #[must_use]
    pub const fn top_right() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Top,
        }
    }

    #[must_use]
    pub const fn bottom_left() -> Self {
        Self {
            horizontal: Horizontal::Left,
            vertical: Vertical::Bottom,
        }
    }

    #[must_use]
    pub const fn bottom_right() -> Self {
        Self {
            horizontal: Horizontal::Right,
            vertical: Vertical::Bottom,
        }
    }
}

impl Default for AnchorOrigin {
    fn default() -> Self {
        Self::top_left()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Horizontal {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Vertical {
    Top,
    Bottom,
}
