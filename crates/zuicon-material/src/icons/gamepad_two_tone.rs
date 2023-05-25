// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GamepadTwoTone)]
pub fn gamepad_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="GamepadTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M6.67 11H4v2h2.67l1-1zM13 6.67V4h-2v2.67l1 1zm-2 10.66V20h2v-2.67l-1-1zM16.33 12l1 1H20v-2h-2.67z" opacity=".3"/><path d="M9 16.5V22h6v-5.5l-3-3-3 3zm4 3.5h-2v-2.67l1-1 1 1V20zm2-12.5V2H9v5.5l3 3 3-3zM11 4h2v2.67l-1 1-1-1V4zM7.5 9H2v6h5.5l3-3-3-3zm-.83 4H4v-2h2.67l1 1-1 1zm9.83-4l-3 3 3 3H22V9h-5.5zm3.5 4h-2.67l-1-1 1-1H20v2z"/>
        </SvgIcon>
    }
}
