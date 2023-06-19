// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::color::Color;
use crate::svg_icon::FontSize;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub base_class_name: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or(Color::Inherit)]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub font_size: FontSize,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Icon)]
pub fn icon(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
