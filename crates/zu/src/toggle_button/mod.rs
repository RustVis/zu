// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{
    function_component, html, AttrValue, Callback, Children, Classes, Html, MouseEvent, Properties,
};

use crate::styles::color::Color;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    #[prop_or(false)]
    pub disable_ripple: bool,

    #[prop_or(false)]
    pub full_width: bool,

    #[prop_or_default]
    pub on_change: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ToggleButton)]
pub fn toggle_button(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
