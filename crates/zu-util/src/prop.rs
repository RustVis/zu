// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::AttrValue;

/// Convert value to optional string.
///
/// Thus ignores `None` attribute in html dom.
pub trait ToAttr {
    fn to_attr(self) -> Option<String>;
}

impl ToAttr for &AttrValue {
    fn to_attr(self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self.as_str().to_owned())
        }
    }
}

impl ToAttr for i32 {
    fn to_attr(self) -> Option<String> {
        if self >= 0 {
            Some(self.to_string())
        } else {
            None
        }
    }
}

impl ToAttr for Option<i32> {
    fn to_attr(self) -> Option<String> {
        self.map(|val| val.to_string())
    }
}

impl ToAttr for bool {
    fn to_attr(self) -> Option<String> {
        if self {
            Some(self.to_string())
        } else {
            None
        }
    }
}
