// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Children, Html, Properties};

use crate::styles::orientation::Orientation;
use crate::styles::text_align::TextAlign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    FullWidth,
    Inset,
    Middle,
}

impl Default for Variant {
    fn default() -> Self {
        Self::FullWidth
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Absolutely position the element.
    #[prop_or(false)]
    pub absolute: bool,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub component: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub orientation: Orientation,

    #[prop_or(TextAlign::Center)]
    pub text_align: TextAlign,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div".to_owned()
    } else {
        props.component.clone()
    };

    html! {
        <@{component}>
        </@>
    }
}
