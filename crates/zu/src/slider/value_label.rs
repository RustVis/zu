// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueLabel {
    Auto,
    On,
    Off,
}

impl Default for ValueLabel {
    fn default() -> Self {
        Self::Off
    }
}
