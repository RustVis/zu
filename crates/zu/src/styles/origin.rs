// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Origin {
    pub horizontal: HorizontalOrigin,
    pub vertical: VerticalOrigin,
}

impl Origin {
    #[must_use]
    pub const fn top_left() -> Self {
        Self {
            horizontal: HorizontalOrigin::Left,
            vertical: VerticalOrigin::Top,
        }
    }

    #[must_use]
    pub const fn top_right() -> Self {
        Self {
            horizontal: HorizontalOrigin::Right,
            vertical: VerticalOrigin::Top,
        }
    }

    #[must_use]
    pub const fn bottom_left() -> Self {
        Self {
            horizontal: HorizontalOrigin::Left,
            vertical: VerticalOrigin::Bottom,
        }
    }
}

impl Default for Origin {
    fn default() -> Self {
        Self::top_left()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HorizontalOrigin {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VerticalOrigin {
    Top,
    Center,
    Bottom,
}
