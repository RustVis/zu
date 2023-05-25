// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlayLessonRounded)]
pub fn play_lesson_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PlayLessonRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,11c0.34,0,0.67,0.03,1,0.08V4c0-1.1-0.9-2-2-2H5C3.9,2,3,2.9,3,4v16c0,1.1,0.9,2,2,2h7.26C11.47,20.87,11,19.49,11,18 C11,14.13,14.13,11,18,11z M7.76,10.55C7.42,10.75,7,10.51,7,10.12V4h5v6.12c0,0.39-0.42,0.63-0.76,0.43L9.5,9.5L7.76,10.55z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M16.75,19.6v-3.2c0-0.39,0.43-0.63,0.76-0.42l2.56,1.6 c0.31,0.2,0.31,0.65,0,0.85l-2.56,1.6C17.18,20.23,16.75,19.99,16.75,19.6z"/>
        </SvgIcon>
    }
}
