// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::Html;

/// Used as badge content.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Str(&'static str),
    String(String),
    Num(i32),
    Node(Html),
}

impl Default for Content {
    fn default() -> Self {
        Self::Num(0)
    }
}
