// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::AttrValue;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    #[default]
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum SeverityColor {
    #[default]
    Success,
    Info,
    Warning,
    Error,
    Custom(AttrValue),
}
