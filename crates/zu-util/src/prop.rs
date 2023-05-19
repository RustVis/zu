// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::AttrValue;

/// Convert `AttrValue` to optional string.
///
/// Thus ignores `None` attribute in html dom.
#[must_use]
pub fn attr_optional(val: &AttrValue) -> Option<String> {
    if val.is_empty() {
        None
    } else {
        Some(val.to_string())
    }
}
