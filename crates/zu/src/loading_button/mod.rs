// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod position;

use yew::{
    function_component, html, AttrValue, Callback, Children, Classes, DragEvent, FocusEvent, Html,
    KeyboardEvent, MouseEvent, Properties, TouchEvent,
};

use crate::styles::button_variant::ButtonVariant;
use crate::styles::color::Color;
use crate::styles::size::Size;
pub use position::Position;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
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
    pub loading: bool,

    #[prop_or_default]
    pub loading_indicator: AttrValue,

    #[prop_or_default]
    pub loading_position: Position,

    #[prop_or_default]
    pub start_icon: Option<Html>,

    #[prop_or_default]
    pub end_icon: Option<Html>,

    /// Default is `Size::Medium`.
    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

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

#[function_component(LoadingButton)]
pub fn loading_button(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
