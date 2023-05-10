// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod color;
pub mod size;

use yew::{classes, Classes};

#[must_use]
pub fn classes_if(list: Vec<(String, bool)>) -> Classes {
    classes!(list
        .into_iter()
        .filter(|item| item.1)
        .map(|item| item.0)
        .collect::<Vec<String>>())
}

pub trait CssClass {
    fn css_class(&self) -> &'static str;
}

pub trait CssValue {
    fn css_value(&self) -> String;
}