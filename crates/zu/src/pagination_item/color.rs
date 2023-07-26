// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardColor {
    Standard,
    Primary,
    Secondary,
}

impl Default for StandardColor {
    fn default() -> Self {
        Self::Standard
    }
}
