// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::type_repetition_in_bounds)]

use yew::{
    function_component, html, AttrValue, Callback, ChildrenWithProps, Classes, Html, MouseEvent,
    Properties,
};

use crate::styles::color::Color;
use crate::styles::orientation::Orientation;
use crate::styles::size::Size;
use crate::toggle_button::ToggleButton;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: ChildrenWithProps<ToggleButton<T>>,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub exclusive: bool,

    #[prop_or(false)]
    pub full_width: bool,

    #[prop_or_default]
    pub on_change: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub orientation: Orientation,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    pub value: T,
}

#[function_component(ToggleButtonGroup)]
pub fn toggle_button_group<T: Clone + PartialEq + 'static>(props: &Props<T>) -> Html {
    html! {
        <div>
            {for props.children.iter()}
        </div>
    }
}
