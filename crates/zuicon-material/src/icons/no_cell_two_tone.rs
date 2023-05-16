// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoCellTwoTone)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NoCellTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M7,21h10v-1H7V21z M7,3v1h10V3H7z" opacity=".3"/><path d="M17,6v8.17l2,2V3c0-1.1-0.9-1.99-2-1.99L7,1C6.15,1,5.42,1.55,5.13,2.3L8.83,6H17z M7,3h10v1H7V3z M21.19,21.19L2.81,2.81 L1.39,4.22L5,7.83V21c0,1.1,0.9,2,2,2h10c0.85,0,1.58-0.55,1.87-1.3l0.91,0.91L21.19,21.19z M17,21H7v-1h10V21z M7,18V9.83 L15.17,18H7z"/>
        </SvgIcon>
    }
}
