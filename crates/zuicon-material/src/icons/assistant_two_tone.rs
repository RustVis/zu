// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AssistantTwoTone)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AssistantTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M9.83 18l.59.59L12 20.17l1.59-1.59.58-.58H19V4H5v14h4.83zm.29-8.88L12 5l1.88 4.12L18 11l-4.12 1.88L12 17l-1.88-4.12L6 11l4.12-1.88z" opacity=".3"/><path d="M5 20h4l3 3 3-3h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2zM5 4h14v14h-4.83l-.59.59L12 20.17l-1.59-1.59-.58-.58H5V4zm7 13l1.88-4.12L18 11l-4.12-1.88L12 5l-1.88 4.12L6 11l4.12 1.88z"/>
        </SvgIcon>
    }
}