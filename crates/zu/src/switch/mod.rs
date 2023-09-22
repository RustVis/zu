// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{
    function_component, html, AttrValue, Callback, Children, Classes, Html, MouseEvent, Properties,
};

use crate::styles::color::Color;
use crate::styles::edge::Edge;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    /// If true, the component is checked.
    #[prop_or(false)]
    pub checked: bool,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub children: Children,

    /// The color of the component.
    #[prop_or_default]
    pub color: Color,

    /// The default checked state. Use when the component is not controlled.
    #[prop_or(false)]
    pub default_checked: bool,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// If given, uses a negative margin to counteract the padding on one side
    #[prop_or_default]
    pub edge: Option<Edge>,

    /// The id of the input element.
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub on_change: Callback<MouseEvent, ()>,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Switch)]
pub fn switch(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
