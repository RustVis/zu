// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod color;
mod size;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::internal::svg_icons::{RadioButtonChecked, RadioButtonUnchecked};
use crate::styles::color::Color;
use crate::styles::size::Size;
use crate::switch_base::{SwitchBase, Variant};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// If `true`, the component is checked.
    #[prop_or(false)]
    pub checked: bool,

    /// The icon to display when the component is checked.
    ///
    /// Default is `<RadioButtonChecked />`
    #[prop_or_default]
    pub checked_icon: Option<Html>,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The color of the component.
    #[prop_or_default]
    pub color: Color,

    /// If `true`, the component is checked by default.
    #[prop_or(false)]
    pub default_checked: bool,

    /// If `true`, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If `true`, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// The icon to display when the component is unchecked.
    ///
    /// Default is `<RadioButtonUnchecked />`.
    #[prop_or_default]
    pub icon: Option<Html>,

    /// The id of the `input` element.
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub input_aria_label: AttrValue,

    /// Name attribute of the `input` element.
    #[prop_or_default]
    pub name: AttrValue,

    /// Callback fired when the state is changed.
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,

    /// If `true`, the `input` element is required.
    #[prop_or(false)]
    pub required: bool,

    /// The size of the component.
    ///
    /// `Size::Small` is equivalent to the dense radio styling.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// The value of the component. The DOM API casts this to a string.
    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(Radio)]
pub fn radio(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuRadio-root",
        color::css_class(props.color),
        size::css_class(props.size),
        if props.disable_ripple {
            ""
        } else {
            "ZuRadio-enableRipple"
        },
        props.classes.clone(),
    );

    let checked_icon = props
        .checked_icon
        .as_ref()
        .map_or_else(|| html! {<RadioButtonChecked />}, Clone::clone);
    let icon = props
        .icon
        .as_ref()
        .map_or_else(|| html! {<RadioButtonUnchecked />}, Clone::clone);

    html! {
        <SwitchBase
            classes={root_cls}
            aria_label={&props.input_aria_label}
            checked={props.checked}
            checked_icon={checked_icon}
            default_checked={props.default_checked}
            disabled={props.disabled}
            disable_focus_ripple={props.disable_ripple}
            icon={icon}
            id={&props.id}
            name={&props.name}
            required={props.required}
            size={props.size}
            style={&props.style}
            value={&props.value}
            variant={Variant::Radio}
        >
        </SwitchBase>
    }
}
