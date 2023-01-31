// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub const UPDATE_KEY: &str = "RUICON_UPDATE";

/// Get inner html of an svg file, withoug <svg> root tag.
pub fn get_svg_inner(s: &str) -> Option<&str> {
    let start_index = match s.find("<svg") {
        Some(index) => index,
        None => return None,
    };
    let end_index = match s.find("</svg>") {
        Some(index) => index,
        None => return None,
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
    Some(&s[start_index_end..end_index].trim())
}

/// Check whether RUICON module shall be refreshed.
///
/// Current `RUICON_REFRESH=1` environment is used.
pub fn need_update() -> bool {
    match std::env::var_os(UPDATE_KEY) {
        Some(val) => !val.is_empty() || true,
        None => false,
    }
}
