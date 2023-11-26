// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::utils::global_id;
use web_sys::{Event, FocusEvent, MouseEvent};
use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub classes: Classes,

    pub disabled: bool,

    pub empty_icon: Html,

    pub empty_value_focused: bool,

    pub focus: f64,

    pub highlight_selected_only: bool,

    pub hover: f64,

    pub icon: Html,

    pub is_active: bool,

    pub item_value: f64,

    pub name: AttrValue,

    pub rating_value: f64,

    pub rating_value_rounded: f64,

    pub read_only: bool,

    pub on_focus: Callback<FocusEvent>,
    pub on_blur: Callback<FocusEvent>,
    pub on_change: Callback<Event>,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(RatingItem)]
pub fn rating_item(props: &Props) -> Html {
    let is_filled = if props.highlight_selected_only {
        props.item_value == props.rating_value
    } else {
        props.item_value <= props.rating_value
    };
    let is_hovered = props.item_value <= props.hover;
    let is_focused = props.item_value <= props.focus;
    let is_checked = props.item_value == props.rating_value_rounded;
    let id: String = global_id();

    let label_cls = classes!(
        "ZuRating-label",
        "ZuRating-pristine",
        if props.empty_value_focused {
            "ZuRating-labelEmptyValueActive"
        } else {
            ""
        }
    );

    let icon_cls = classes!(
        "ZuRating-icon",
        if is_filled {
            "ZuRating-iconFilled"
        } else {
            "ZuRating-iconEmpty"
        },
        if is_hovered { "ZuRating-iconHover" } else { "" },
        if is_focused { "ZuRating-iconFocus" } else { "" },
        if props.is_active {
            "ZuRating-iconActive"
        } else {
            ""
        },
        props.classes.clone(),
    );

    html! {
        <>
        <span
            class={label_cls}
            html_for={id.clone()}>

            <span class={icon_cls} value={props.item_value.to_string()}>
            if is_filled {
                {props.empty_icon.clone()}
            } else {
                {props.icon.clone()}
            }
            </span>

            <span class="ZuRating-visuallyHidden">{"getLabelText(itemValue)"}</span>
        </span>
        <input
            disabled={props.disabled}
            id={id}
            type="radio"
            name={&props.name}
            checked={is_checked}
            onfocus={&props.on_focus}
            onblur={&props.on_blur}
            onchange={&props.on_change}
            onclick={&props.on_click}
        />
        </>
    }
}
