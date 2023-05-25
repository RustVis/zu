// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfyOutlined)]
pub fn view_comfy_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ViewComfyOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M2,4v16h20V4H2z M4,6h16v5H4V6z M4,18v-5h4v5H4z M10,18v-5h10v5H10z"/>
        </SvgIcon>
    }
}
