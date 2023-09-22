// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod color;
mod position;
mod size;
mod variant;

use yew::{
    classes, function_component, html, AttrValue, Callback, Children, Classes, DragEvent,
    FocusEvent, Html, KeyboardEvent, MouseEvent, Properties, TouchEvent,
};

use crate::button::Button;
use crate::circular_progress::{CircularProgress, Size as CircularProgressSize};
use crate::styles::button_variant::ButtonVariant;
use crate::styles::color::Color;
use crate::styles::size::Size;
pub use position::Position;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

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
    pub disable_elevation: bool,

    #[prop_or(false)]
    pub full_width: bool,

    /// If `true`, the loading indicator is shown.
    ///
    /// Default is false.
    #[prop_or(false)]
    pub loading: bool,

    /// Element placed before the children if the button is in loading state.
    #[prop_or_default]
    pub loading_indicator: Option<Html>,

    #[prop_or(Position::Center)]
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
    pub tab_index: Option<i32>,

    #[prop_or(ButtonVariant::Text)]
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

fn create_indicator(
    indicator: &Option<Html>,
    position: Position,
    button_size: Size,
    button_variant: ButtonVariant,
    full_width: bool,
) -> Html {
    let root_cls = classes!(
        "ZuLoadingButton-loadingIndicator",
        position.indicator_class(),
        size::indicator_class(button_size),
        variant::indicator_class(button_variant),
        if full_width {
            "ZuLoadingButton-loadingIndicatorFullWidth"
        } else {
            ""
        }
    );

    if let Some(indicator) = indicator {
        html! {
            <span class={root_cls}>
                {indicator.clone()}
            </span>
        }
    } else {
        html! {
            <div class={root_cls}>
                <CircularProgress color={Color::Inherit} size={CircularProgressSize::Num(16)} />
            </div>
        }
    }
}

#[function_component(LoadingButton)]
pub fn loading_button(props: &Props) -> Html {
    if props.start_icon.is_some() && props.loading_position != Position::Start {
        log::error!("The loading_position='start' should be used in combination with start_icon.");
    }
    if props.end_icon.is_some() && props.loading_position != Position::End {
        log::error!("The loading_position='end' should be used in combination with end_icon.");
    }

    let root_cls = classes!(
        "ZuLoadingButton-root",
        if props.loading && props.loading_position == Position::Center {
            "ZuLoadingButton-loading"
        } else {
            ""
        },
        if props.disable_elevation {
            "ZuLoadingButton-disableElevation"
        } else {
            ""
        },
        if props.full_width {
            "ZuLoadingButton-fullWidth"
        } else {
            ""
        },
        props.classes.clone(),
    );

    let indicator = if props.loading {
        create_indicator(
            &props.loading_indicator,
            props.loading_position,
            props.size,
            props.variant,
            props.full_width,
        )
    } else {
        html! {}
    };

    html! {
        <Button
            classes={root_cls}
            aria_label={&props.aria_label}
            disabled={props.disabled || props.loading}
            size={props.size}
            start_icon={props.start_icon.clone()}
            start_icon_classes={classes!("ZuLoadingButton-startIconLoadingStart")}
            end_icon={props.end_icon.clone()}
            end_icon_classes={classes!("ZuLoadingButton-endIconLoadingEnd")}
            tab_index={props.tab_index}
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
            >

            if props.loading_position == Position::End {
                {for props.children.iter()}
                {indicator}
            } else {
                {indicator}
                {for props.children.iter()}
            }

        </Button>
    }
}
