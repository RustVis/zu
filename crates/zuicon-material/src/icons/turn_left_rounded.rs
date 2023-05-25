// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnLeftRounded)]
pub fn turn_left_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TurnLeftRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M7.71,13.29c-0.39,0.39-1.02,0.39-1.41,0l-2.59-2.59c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0 c0.39,0.39,0.39,1.02,0,1.41L6.83,9L15,9c1.1,0,2,0.9,2,2v8c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1v-8l-8.17,0l0.88,0.88 C8.1,12.27,8.1,12.9,7.71,13.29z"/>
        </SvgIcon>
    }
}
