// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShortTextOutlined)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ShortTextOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0z" fill="none"/><path d="M4 9h16v2H4V9zm0 4h10v2H4v-2z"/>
        </SvgIcon>
    }
}