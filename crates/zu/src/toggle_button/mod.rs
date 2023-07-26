// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::type_repetition_in_bounds)]

use yew::{
    function_component, html, AttrValue, Callback, Children, Classes, Html, MouseEvent, Properties,
};

use crate::button_base::ButtonBase;
use crate::styles::color::Color;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<T>
where
    T: Clone + PartialEq,
{
    #[prop_or_default]
    pub aria_label: AttrValue,

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

    pub value: T,
}

#[function_component(ToggleButton)]
pub fn toggle_button<T>(props: &Props<T>) -> Html
where
    T: Clone + PartialEq,
{
    html! {
        <ButtonBase
            aria_label={props.aria_label.clone()}
            disabled={props.disabled}
            focus_ripple={!props.disable_focus_ripple}
            >
        </ButtonBase>
    }
}
