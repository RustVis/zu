// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use yew::{Children, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LabelProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub required: bool,

    #[prop_or_default]
    pub size: LabelSize,

    #[prop_or_default]
    pub weight: LabelWeight,

    pub children: Children,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelSize {
    Small,
    Medium,
    Large,
}

impl Default for LabelSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelWeight {
    Regular,
    SemiBold,
}

impl Default for LabelWeight {
    fn default() -> Self {
        Self::Regular
    }
}
