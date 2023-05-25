// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SixtyFpsRounded)]
pub fn sixty_fps_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SixtyFpsRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M19,8v8h-4V8H19 M19,5h-4c-1.66,0-3,1.34-3,3v8c0,1.66,1.34,3,3,3h4c1.66,0,3-1.34,3-3V8C22,6.34,20.66,5,19,5z M10,6.5 L10,6.5C10,5.67,9.33,5,8.5,5H5C3.34,5,2,6.34,2,8v8c0,1.66,1.34,3,3,3h3c1.66,0,3-1.34,3-3v-3c0-1.66-1.34-3-3-3H5V8h3.5 C9.33,8,10,7.33,10,6.5z M8,13v3H5v-3H8z"/>
        </SvgIcon>
    }
}
