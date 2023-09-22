// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::AttrValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Success,
    Info,
    Warning,
    Error,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Success
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeverityColor {
    Success,
    Info,
    Warning,
    Error,
    Custom(AttrValue),
}

impl Default for SeverityColor {
    fn default() -> Self {
        Self::Success
    }
}
