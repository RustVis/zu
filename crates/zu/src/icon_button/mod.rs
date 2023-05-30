// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod edge;
mod size;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::button_base::ButtonBase;
use crate::styles::color::Color;
use crate::styles::edge::Edge;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or(Color::Default)]
    pub color: Color,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    #[prop_or(false)]
    pub disable_ripple: bool,

    #[prop_or_default]
    pub edge: Option<Edge>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(IconButton)]
pub fn icon_button(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuIconButton-root",
        if props.disabled {
            "ZuIconButton-disabled"
        } else {
            ""
        },
        color::css_class(&props.color),
        edge::css_class(props.edge),
        size::css_class(props.size),
    );

    html! {
        <ButtonBase classes={root_cls}
            aria_label={&props.aria_label}
            style={&props.style}>
        </ButtonBase>
    }
}
