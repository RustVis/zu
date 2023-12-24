// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Placement {
    BottomEnd,
    BottomStart,
    Bottom,
    LeftEnd,
    LeftStart,
    #[default]
    Left,
    RightEnd,
    RightStart,
    Right,
    TopEnd,
    TopStart,
    Top,
}
