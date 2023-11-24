// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod color;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::internal::svg_icons::{CheckBox, CheckBoxOutlineBlank, IntermediateCheckBox};
use crate::styles::color::Color;
use crate::styles::size::Size;
use crate::switch_base::{SwitchBase, Variant};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    /// If `true`, the component is checked.
    #[prop_or(false)]
    pub checked: bool,

    /// The icon to display when the component is checked.
    ///
    /// Default is CheckBox icon.
    #[prop_or_default]
    pub checked_icon: Option<Html>,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The color of the component.
    #[prop_or_default]
    pub color: Color,

    /// The default checked state. Use when the component is not controlled.
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
    /// Default is CheckBoxOutlineBlank
    #[prop_or_default]
    pub icon: Option<Html>,

    /// The id of the input element.
    #[prop_or_default]
    pub id: AttrValue,

    /// If true, the component appears indeterminate.
    #[prop_or(false)]
    pub indeterminate: bool,

    /// Default is IndeterminateCheckBox
    #[prop_or_default]
    pub indeterminate_icon: Option<Html>,

    /// Callback fired when the state is changed.
    #[prop_or_default]
    pub on_change: Option<Callback<bool>>,

    /// If true, the input element is required.
    #[prop_or(false)]
    pub required: bool,

    /// The size of the component.
    ///
    /// `Size::small` is equivalent to the dense checkbox styling.
    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// The value of the component.
    ///
    /// The browser uses "on" as the default value.
    #[prop_or_default]
    pub value: String,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuCheckbox-root",
        if props.indeterminate {
            "ZuCheckbox-indeterminate"
        } else {
            ""
        },
        color::css_class(props.color),
        if props.disable_ripple {
            ""
        } else {
            "ZuCheckbox-enableRipple"
        },
        props.classes.clone(),
    );

    let icon = props
        .icon
        .as_ref()
        .map_or_else(|| html! {<CheckBoxOutlineBlank />}, Clone::clone);

    let checked_icon = if props.indeterminate {
        props
            .indeterminate_icon
            .as_ref()
            .map_or_else(|| html! {<IntermediateCheckBox />}, Clone::clone)
    } else {
        html! {<CheckBox />}
    };

    // TODO(Shaohua): Support indeterminate

    html! {
        <SwitchBase
            aria_label={&props.aria_label}
            classes={root_cls}
            checked={props.checked}
            checked_icon={checked_icon}
            default_checked={props.default_checked}
            disabled={props.disabled}
            icon={icon}
            on_change={props.on_change.clone()}
            style={&props.style}
            variant={Variant::Checkbox}
            >
        </SwitchBase>
    }
}
