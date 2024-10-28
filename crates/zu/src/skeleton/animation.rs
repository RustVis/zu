// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Animation {
    #[default]
    Pulse,
    Wave,
    None,
}

impl CssClass for Animation {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Pulse => "ZuSkeleton-pulse",
            Self::Wave => "ZuSkeleton-wave",
            Self::None => "",
        }
    }
}
