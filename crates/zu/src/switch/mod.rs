// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod color;
mod edge;
mod size;

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, Html, Properties,
};

use crate::styles::color::Color;
use crate::styles::edge::Edge;
use crate::styles::size::Size;
use crate::switch_base::{SwitchBase, Variant};

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
    pub on_change: Option<Callback<bool>>,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Switch)]
pub fn switch(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuSwitch-root",
        edge::css_class(props.edge),
        size::css_class(props.size),
        props.classes.clone(),
    );
    let base_cls = classes!("ZuSwitch-switchBase", color::color_class(props.color),);
    let base_input_cls = classes!("ZuSwitch-input");
    log::info!("default checked: {}", props.default_checked);

    let icon = html! {
        <span class="ZuSwitch-thumb">
        </span>
    };

    html! {
        <span class={root_cls}>
            <SwitchBase
                aria_label={&props.aria_label}
                classes={base_cls}
                checked={props.checked}
                default_checked={props.default_checked}
                input_classes={base_input_cls}
                icon={icon.clone()}
                on_change={props.on_change.clone()}
                checked_icon={icon}
                variant={Variant::Switch}
                >
            </SwitchBase>
             <span class="ZuSwitch-track">
            </span>
        </span>
    }
}
