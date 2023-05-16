// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfyTwoTone)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ViewComfyTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M10,18h10v-5H10V18z M4,6v5h16V6H4z M4,18h4v-5H4V18z" opacity=".3"/><path d="M2,4v16h20V4H2z M8,18H4v-5h4V18z M20,18H10v-5h10V18z M20,11H4V6h16V11z"/>
        </SvgIcon>
    }
}
