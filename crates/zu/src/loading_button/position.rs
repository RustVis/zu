// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Position {
    Start,
    End,
    #[default]
    Center,
}

impl Position {
    #[must_use]
    pub const fn root_class(self) -> &'static str {
        match self {
            Self::Start => "ZuLoadingButton-loadingPositionStart",
            Self::End => "ZuLoadingButton-loadingPositionEnd",
            Self::Center => "ZuLoadingButton-loadingPositionCenter",
        }
    }

    #[must_use]
    pub const fn indicator_class(self) -> &'static str {
        match self {
            Self::Start => "ZuLoadingButton-loadingIndicatorStart",
            Self::End => "ZuLoadingButton-loadingIndicatorEnd",
            Self::Center => "ZuLoadingButton-loadingIndicatorCenter",
        }
    }

    #[must_use]
    pub const fn start_icon_class(self) -> &'static str {
        match self {
            Self::Start => "ZuLoadingButton-startIconLoadingStart",
            Self::End => "ZuLoadingButton-startIconLoadingEnd",
            Self::Center => "ZuLoadingButton-startIconLoadingCenter",
        }
    }

    #[must_use]
    pub const fn end_icon_class(self) -> &'static str {
        match self {
            Self::Start => "ZuLoadingButton-endIconLoadingStart",
            Self::End => "ZuLoadingButton-endIconLoadingEnd",
            Self::Center => "ZuLoadingButton-endIconLoadingCenter",
        }
    }
}
