// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Used as abstract component for Checkbox, Switch and Radio.

mod edge;
mod size;
mod variant;

use yew::{
    classes, function_component, html, AttrValue, Callback, Classes, Html, MouseEvent, Properties,
};
use zu_util::prop::ToAttr;

use crate::button_base::ButtonBase;
use crate::styles::edge::Edge;
use crate::styles::size::Size;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub auto_focus: bool,

    #[prop_or(false)]
    pub checked: bool,

    pub checked_icon: Html,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub default_checked: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    #[prop_or_default]
    pub edge: Option<Edge>,

    pub icon: Html,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub name: AttrValue,

    pub on_blur: Callback<(), ()>,

    pub on_change: Callback<MouseEvent, ()>,

    pub on_focus: Callback<(), ()>,

    #[prop_or(false)]
    pub read_only: bool,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub tab_index: Option<i32>,

    pub variant: Variant,

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

    html! {
        <ButtonBase classes={root_cls}
            component="span"
            disabled={props.disabled}
            disable_focus_ripple={props.disable_focus_ripple}
        >
            <input class="ZuSwitchBase-input"
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
