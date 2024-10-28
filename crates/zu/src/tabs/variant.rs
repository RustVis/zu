// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Variant {
    #[default]
    Standard,
    FullWidth,
    Scrollable,
}
