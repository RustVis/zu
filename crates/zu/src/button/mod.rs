// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod size;
mod variant;

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, DragEvent, FocusEvent, Html,
    KeyboardEvent, MouseEvent, Properties, TouchEvent,
};

use crate::button_base::ButtonBase;
use crate::styles::{color::Color, size::Size, CssClass};
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, no elevation is used.
    #[prop_or(false)]
    pub disable_elevation: bool,

    /// If true, the keyboard focus ripple is disabled.
    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// Element placed after the children.
    #[prop_or_default]
    pub end_icon: Option<Html>,

    /// If true, the button will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The URL to link to when the button is clicked.
    ///
    ///  If defined, an a element will be used as the root node.
    #[prop_or_default]
    pub href: AttrValue,

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

    /// The size of the component.
    ///
    ///  small is equivalent to the dense button styling.
    #[prop_or_default]
    pub size: Size,

    /// Element placed before the children.
    #[prop_or_default]
    pub start_icon: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(-1)]
    pub tab_index: i32,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    // TODO(Shaohua): Add mouse event callbacks

    let root_cls = classes!(
        "ZuButton-root",
        props.classes.as_str().to_owned(),
        props.variant.css_class(),
        color::color_class(&props.color),
        size::css_class(props.size),
        if props.disable_elevation {
            "ZuButton-disable_elevation"
        } else {
            ""
        },
        if props.full_width {
            "ZuButton-fullWidth"
        } else {
            ""
        }
    );

    //let label_cls = "ZuButton-label";
    //let start_icon_cls = classes!("ZuButton-startIcon", props.size.icon_class(),);
    //let end_icon_cls = classes!("ZuButton-endIcon", props.size.icon_class(),);

    // TODO(Shaohua): Set class for start_icon and end_icon.

    html! {
        <ButtonBase
            classes={root_cls}
            on_blur={props.on_blur.clone()}
            on_click={props.on_click.clone()}
            on_context_menu={props.on_context_menu.clone()}
            on_drag_leave={props.on_drag_leave.clone()}
            on_focus={props.on_focus.clone()}
            on_focus_visible={props.on_focus_visible.clone()}
            on_key_down={props.on_key_down.clone()}
            on_key_up={props.on_key_up.clone()}
            on_mouse_down={props.on_mouse_down.clone()}
            on_mouse_enter={props.on_mouse_enter.clone()}
            on_mouse_leave={props.on_mouse_leave.clone()}
            on_mouse_up={props.on_mouse_up.clone()}
            on_touch_end={props.on_touch_end.clone()}
            on_touch_move={props.on_touch_move.clone()}
            on_touch_start={props.on_touch_start.clone()}
            tab_index={props.tab_index}
            >
            if let Some(start_icon) = &props.start_icon {
                {start_icon.clone()}
            }
            {for props.children.iter()}
            if let Some(end_icon) = &props.end_icon {
                {end_icon.clone()}
            }
        </ButtonBase>
    }
}
