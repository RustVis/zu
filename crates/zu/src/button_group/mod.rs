// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::rc::Rc;
use yew::{
    classes, function_component, html, AttrValue, ChildrenWithProps, Classes, Html, Properties,
};
use zu_util::prop::ToAttr;

use crate::button::{Button, Props as ButtonProps};
use crate::styles::button_variant::ButtonVariant;
use crate::styles::color::Color;
use crate::styles::orientation::Orientation;
use crate::styles::size::Size;

const BUTTON_CLS: &str = "ZuButtonGroup-button";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: ChildrenWithProps<Button>,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(Color::Primary)]
    pub color: Color,

    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, no elevation is used.
    #[prop_or(false)]
    pub disable_elevation: bool,

    /// If true, the button keyboard focus ripple is disabled.
    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    /// If true, the button ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// If true, the buttons will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The component orientation (layout flow direction).
    #[prop_or(Orientation::Horizontal)]
    pub orientation: Orientation,

    /// The size of the component. small is equivalent to the dense button styling.
    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// The variant to use.
    #[prop_or(ButtonVariant::Outlined)]
    pub variant: ButtonVariant,
}

#[function_component(ButtonGroup)]
pub fn button_group(props: &Props) -> Html {
    let root_cls = classes!("ZuButtonGroup-root", props.classes.clone(),);

    let children: Vec<_> = props
        .children
        .iter()
        .map(|mut item| {
            let p = Rc::<ButtonProps>::make_mut(&mut item.props);
            p.classes.push(BUTTON_CLS);
            p.color = props.color;
            p.disabled = props.disabled;
            p.disable_elevation = props.disable_elevation;
            p.disable_focus_ripple = props.disable_focus_ripple;
            p.disable_ripple = props.disable_ripple;
            p.full_width = props.full_width;
            p.size = props.size;
            p.variant = props.variant;
            item
        })
        .collect();
    let children = ChildrenWithProps::new(children);

    html! {
        <@{props.component.to_string()}
            aria_label={props.aria_label.to_attr()}
            class={root_cls}
            style={props.style.clone()}
            >
            {for children.iter()}
        </@>
    }
}
