// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

pub mod anchor_origin;
pub mod button_variant;
pub mod color;
pub mod direction;
pub mod edge;
pub mod flex_direction;
pub mod flex_wrap;
pub mod input_type;
pub mod item_align;
pub mod label_variant;
pub mod margin;
pub mod orientation;
pub mod placement;
pub mod position;
pub mod severity;
pub mod shape_variant;
pub mod side;
pub mod size;
pub mod sort_direction;
pub mod spacing;
pub mod text_align;
pub mod transition_duration;

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
    fn css_value(&self) -> &'static str;
}
