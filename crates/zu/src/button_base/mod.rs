// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! `ButtonBase` contains as few styles as possible.
//!
//! It aims to be a simple building block for creating a button.
//! It contains a load of style reset and some focus/ripple logic.

use yew::{
    classes, function_component, html, use_state, AttrValue, Callback, Children, DragEvent,
    FocusEvent, Html, KeyboardEvent, MouseEvent, Properties, TouchEvent,
};
use zu_util::prop::ToAttr;

pub const DEFAULT_LINK_COMPONENT: &str = "a";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    // TODO(Shaohua): Add action ref.
    /// If true, the ripples are centered.
    #[prop_or(false)]
    pub center_ripple: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// If true, the touch ripple effect is disabled.
    #[prop_or(false)]
    pub disable_touch_ripple: bool,

    /// If true, the keyboard focus ripple is disabled.
    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    /// This prop can help identify which element has keyboard focus.
    #[prop_or_default]
    pub focus_visible_class: AttrValue,

    #[prop_or_default]
    pub href: AttrValue,

    /// The component used to render a link when the href prop is provided.
    ///
    /// Default is 'a'
    #[prop_or_default]
    pub link_component: AttrValue,

    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent, ()>>,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_context_menu: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_drag_leave: Option<Callback<DragEvent, ()>>,

    #[prop_or_default]
    pub on_focus: Option<Callback<FocusEvent, ()>>,

    /// Callback fired when the component is focused with a keyboard.
    #[prop_or_default]
    pub on_focus_visible: Option<Callback<FocusEvent, ()>>,

    #[prop_or_default]
    pub on_key_down: Option<Callback<KeyboardEvent, ()>>,

    #[prop_or_default]
    pub on_key_up: Option<Callback<KeyboardEvent, ()>>,

    #[prop_or_default]
    pub on_mouse_down: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_mouse_enter: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_mouse_leave: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_mouse_up: Option<Callback<MouseEvent, ()>>,

    #[prop_or_default]
    pub on_touch_end: Option<Callback<TouchEvent, ()>>,

    #[prop_or_default]
    pub on_touch_move: Option<Callback<TouchEvent, ()>>,

    #[prop_or_default]
    pub on_touch_start: Option<Callback<TouchEvent, ()>>,

    // pub onFocusVisible:
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(-1)]
    pub tab_index: i32,

    // TODO(Shaohua): Replace with a struct.
    #[prop_or_default]
    pub touch_ripple_props: AttrValue,
    // TODO(Shaohua): Add touchRippleRef
}

#[function_component(ButtonBase)]
pub fn button_base(props: &Props) -> Html {
    let focus_visible = use_state(|| false);

    let root_cls = classes!(
        "ZuButtonBase-root",
        if props.disabled {
            "ZuButtonBase-disabled"
        } else {
            ""
        },
        if *focus_visible {
            "ZuButtonBase-focusVisible"
        } else {
            ""
        },
        if *focus_visible {
            props.focus_visible_class.as_str().to_owned()
        } else {
            String::new()
        },
        props.classes.as_str().to_owned(),
        // TODO(Shaohua): Merge into Typography.
        "ZuTypography-button",
    );

    // TODO(Shaohua): Handle mouse events.

    let component = if props.component.is_empty() {
        "button"
    } else {
        props.component.as_str()
    };

    // TODO(Shaohua): Add touch ripple.
    // TODO(Shaohua): Bind on_focus_visible

    let tab_index = if props.disabled { -1 } else { props.tab_index };

    html! {
        <@{component.to_owned()} class={root_cls}
            aria-label={props.aria_label.to_attr()}
            disabled={props.disabled}
            onblur={props.on_blur.clone()}
            onclick={props.on_click.clone()}
            oncontextmenu={props.on_context_menu.clone()}
            ondragleave={props.on_drag_leave.clone()}
            onfocus={props.on_focus.clone()}
            onkeydown={props.on_key_down.clone()}
            onkeyup={props.on_key_up.clone()}
            onmousedown={props.on_mouse_down.clone()}
            onmouseenter={props.on_mouse_enter.clone()}
            onmouseleave={props.on_mouse_leave.clone()}
            onmouseup={props.on_mouse_up.clone()}
            ontouchend={props.on_touch_end.clone()}
            ontouchmove={props.on_touch_move.clone()}
            ontouchstart={props.on_touch_start.clone()}
            tab_index={tab_index.to_string()}
            >
            {for props.children.iter()}
        </@>
    }
}
