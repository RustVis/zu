// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

//! Used as abstract component for Checkbox, Switch and Radio.

mod edge;
mod size;
mod variant;

use yew::{
    classes, function_component, html, AttrValue, Callback, Classes, FocusEvent, Html, MouseEvent,
    Properties,
};
use zu_util::prop::ToAttr;

use crate::button_base::ButtonBase;
use crate::styles::edge::Edge;
use crate::styles::size::Size;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// If `true`, the `input` element is focused during the first mount.
    #[prop_or(false)]
    pub auto_focus: bool,

    /// If `true`, the component is checked.
    #[prop_or(false)]
    pub checked: bool,

    /// The icon to display when the component is checked.
    #[prop_or_default]
    pub checked_icon: Html,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub input_classes: Classes,

    #[prop_or(false)]
    pub default_checked: bool,

    /// If `true`, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(true)]
    pub disable_focus_ripple: bool,

    /// If given, uses a negative margin to counteract the padding on one side.
    #[prop_or_default]
    pub edge: Option<Edge>,

    /// The icon to display when the component is unchecked.
    pub icon: Html,

    /// The id of the `input` element.
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent>>,

    /// Callback fired when the state is changed.
    #[prop_or_default]
    pub on_change: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_focus: Option<Callback<FocusEvent>>,

    /// It prevents the user from changing the value of the field.
    #[prop_or(false)]
    pub read_only: bool,

    /// If `true`, the `input` element is required.
    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub tab_index: Option<i32>,

    /// Switch variant type.
    #[prop_or_default]
    pub variant: Variant,

    /// The value of component.
    #[prop_or_default]
    pub value: Option<String>,
}

#[function_component(SwitchBase)]
pub fn switch_base(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuSwitchBase-root",
        if props.checked {
            "ZuSwitchBase-checked"
        } else {
            ""
        },
        if props.disabled {
            "ZuSwitchBase-disabled"
        } else {
            ""
        },
        edge::css_class(props.edge),
        size::css_class(props.size),
        props.classes.clone(),
    );

    let has_label_for = match props.variant {
        Variant::Checkbox | Variant::Radio => true,
        Variant::Switch => false,
    };

    let input_id = if has_label_for {
        Some(props.id.as_str().to_owned())
    } else {
        None
    };

    let value = if props.variant == Variant::Checkbox {
        props.value.clone()
    } else {
        None
    };

    let input_cls = classes!("ZuSwitchBase-input", props.input_classes.clone());

    html! {
        <ButtonBase classes={root_cls}
            component="span"
            disabled={props.disabled}
            focus_ripple={!props.disable_focus_ripple}
            on_focus={&props.on_focus}
            on_blur={&props.on_blur}
        >
            <input class={input_cls}
                auto_focus={props.auto_focus.to_attr()}
                checked={props.checked}
                default_checked={props.default_checked.to_attr()}
                disabled={props.disabled}
                id={input_id}
                required={props.required}
                tab_index={props.tab_index.to_attr()}
                type={props.variant.name()}
                {value}
            />
            if props.checked {
                {props.checked_icon.clone()}
            } else {
                {props.icon.clone()}
            }
        </ButtonBase>
    }
}
