// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

pub const UPDATE_KEY: &str = "ZUICON_UPDATE";

/// Get inner html of an svg file, withoug `<svg>` root tag.
#[must_use]
pub fn get_svg_inner(s: &str) -> Option<&str> {
    let Some(start_index) = s.find("<svg") else {
        return None
    };
    let Some(end_index) = s.find("</svg>") else {
        return None
    };

    let mut start_index_end = start_index;
    for (index, c) in s[start_index..].chars().enumerate() {
        if c == '>' {
            start_index_end = start_index + index + 1;
            break;
        }
    }
    if start_index == start_index_end || start_index_end >= end_index {
        return None;
    }
    Some(s[start_index_end..end_index].trim())
}

/// Check whether ZUICON modules shall be refreshed.
///
/// Current `ZUICON_UPDATE=1` environment is used.
#[must_use]
pub fn need_update() -> bool {
    std::env::var_os(UPDATE_KEY).map_or(false, |val| !val.is_empty())
}

pub const TEMPLATE_FILE: &str = include_str!("template.rs");
