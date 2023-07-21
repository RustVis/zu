// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod edge;
mod size;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::button_base::ButtonBase;
use crate::styles::color::Color;
use crate::styles::edge::Edge;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub center_ripple: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `Color::Default`.
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

    /// Default is `Size::Medium`.
    #[prop_or(Size::Medium)]
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
        if props.disable_ripple {
            ""
        } else {
            "ZuIconButton-enableRipple"
        },
        color::css_class(props.color),
        edge::css_class(props.edge),
        size::css_class(props.size),
        props.classes.clone(),
    );

    html! {
        <ButtonBase classes={root_cls}
            aria_label={&props.aria_label}
            center_ripple={props.center_ripple}
            disabled={props.disabled}
            focus_ripple={!props.disable_focus_ripple}
            style={props.style.clone()}>
            {for props.children.iter()}
        </ButtonBase>
    }
}
