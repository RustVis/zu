// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

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
    fn to_cls(&self) -> &'static str;
}
