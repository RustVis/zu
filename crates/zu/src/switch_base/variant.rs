// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Checkbox,
    Radio,
    Switch,
}

impl Variant {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Checkbox | Self::Switch => "checkbox",
            Self::Radio => "radio",
        }
    }
}
