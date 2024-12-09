// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod color;
mod size;
mod variant;

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, DragEvent,
    FocusEvent, Html, KeyboardEvent, MouseEvent, Properties, TouchEvent,
};

use crate::button_base::{ButtonBase, ButtonType};
use crate::styles::button_variant::ButtonVariant;
use crate::styles::{color::Color, size::Size};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub button_type: ButtonType,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or(AttrValue::from("button"))]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, no elevation is used.
    #[prop_or(false)]
    pub disable_elevation: bool,

    /// If true, the keyboard focus ripple is disabled.
    #[prop_or(true)]
    pub disable_focus_ripple: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// Element placed before the children.
    #[prop_or_default]
    pub start_icon: Option<Html>,

    #[prop_or_default]
    pub start_icon_classes: Classes,

    /// Element placed after the children.
    #[prop_or_default]
    pub end_icon: Option<Html>,

    #[prop_or_default]
    pub end_icon_classes: Classes,

    /// If true, the button will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The URL to link to when the button is clicked.
    ///
    ///  If defined, an a element will be used as the root node.
    #[prop_or_default]
    pub href: AttrValue,

    /// The size of the component.
    ///
    /// `Small` is equivalent to the dense button styling.
    ///
    /// Default is `Medium`.
    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub tab_index: Option<i32>,

    /// Default value is `Text`.
    #[prop_or_default]
    pub variant: ButtonVariant,

    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent>>,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_context_menu: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_drag_leave: Option<Callback<DragEvent>>,

    #[prop_or_default]
    pub on_focus: Option<Callback<FocusEvent>>,

    /// Callback fired when the component is focused with a keyboard.
    #[prop_or_default]
    pub on_focus_visible: Option<Callback<FocusEvent>>,

    #[prop_or_default]
    pub on_key_down: Option<Callback<KeyboardEvent>>,

    #[prop_or_default]
    pub on_key_up: Option<Callback<KeyboardEvent>>,

    #[prop_or_default]
    pub on_mouse_down: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_mouse_enter: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_mouse_leave: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_mouse_up: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_touch_end: Option<Callback<TouchEvent>>,

    #[prop_or_default]
    pub on_touch_move: Option<Callback<TouchEvent>>,

    #[prop_or_default]
    pub on_touch_start: Option<Callback<TouchEvent>>,
}

fn create_icon(
    is_start: bool,
    icon: Option<&Html>,
    button_size: Size,
    custom_classes: Classes,
) -> Html {
    let icon_cls = classes!(
        if is_start {
            "ZuButton-startIcon"
        } else {
            "ZuButton-endIcon"
        },
        size::icon_class(button_size),
        custom_classes,
    );
    icon.as_ref().map_or_else(
        || html! {},
        |icon| {
            html! {
                <span class={icon_cls}>
                    {icon.clone()}
                </span>
            }
        },
    )
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    // TODO(Shaohua): Add mouse event callbacks
    // TODO(Shaohua): Fix disableElevation not working

    let root_cls = classes!(
        "ZuButton-root",
        variant::css_class(props.variant),
        size::css_class(props.size),
        if props.disable_elevation {
            "ZuButton-disableElevation"
        } else {
            ""
        },
        if props.disabled {
            "ZuButton-disabled"
        } else {
            color::color_class(props.color)
        },
        if props.full_width {
            "ZuButton-fullWidth"
        } else {
            ""
        },
        props.classes.clone(),
    );

    html! {
        <ButtonBase
            classes={root_cls}
            aria_label={&props.aria_label}
            button_type={props.button_type}
            component={&props.component}
            disabled={props.disabled}
            disable_ripple={props.disable_ripple}
            focus_ripple={props.disable_focus_ripple}
            style={&props.style}
            tab_index={props.tab_index}
            on_blur={&props.on_blur}
            on_click={&props.on_click}
            on_context_menu={&props.on_context_menu}
            on_drag_leave={&props.on_drag_leave}
            on_focus={&props.on_focus}
            on_focus_visible={&props.on_focus_visible}
            on_key_down={&props.on_key_down}
            on_key_up={&props.on_key_up}
            on_mouse_down={&props.on_mouse_down}
            on_mouse_enter={&props.on_mouse_enter}
            on_mouse_leave={&props.on_mouse_leave}
            on_mouse_up={&props.on_mouse_up}
            on_touch_end={&props.on_touch_end}
            on_touch_move={&props.on_touch_move}
            on_touch_start={&props.on_touch_start}>
            {create_icon(true, &props.start_icon, props.size, props.start_icon_classes.clone())}
            {for props.children.iter()}
            {create_icon(false, &props.end_icon, props.size, props.end_icon_classes.clone())}
        </ButtonBase>
    }
}
