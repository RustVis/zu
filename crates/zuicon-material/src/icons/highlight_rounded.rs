// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HighlightRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="HighlightRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0z" fill="none"/><path d="M6.29 14.29L9 17v4c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-4l2.71-2.71c.19-.19.29-.44.29-.71V10c0-.55-.45-1-1-1H7c-.55 0-1 .45-1 1v3.59c0 .26.11.52.29.7zM12 2c.55 0 1 .45 1 1v1c0 .55-.45 1-1 1s-1-.45-1-1V3c0-.55.45-1 1-1zM4.21 5.17c.39-.39 1.02-.39 1.42 0l.71.71c.39.39.39 1.02 0 1.41-.39.39-1.02.39-1.41 0l-.72-.71c-.39-.39-.39-1.02 0-1.41zm13.46.71l.71-.71c.39-.39 1.02-.39 1.41 0 .39.39.39 1.02 0 1.41l-.71.71c-.39.39-1.02.39-1.41 0-.39-.39-.39-1.02 0-1.41z"/>
        </SvgIcon>
    }
}