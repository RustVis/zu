// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddModeratorOutlined)]
pub fn add_moderator_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AddModeratorOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M6,11.09v-4.7l6-2.25l6,2.25v3.69c0.71,0.1,1.38,0.31,2,0.6V5l-8-3L4,5v6.09c0,5.05,3.41,9.76,8,10.91 c0.03-0.01,0.05-0.02,0.08-0.02c-0.79-0.78-1.4-1.76-1.75-2.84C7.76,17.53,6,14.42,6,11.09z"/><path d="M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S19.76,12,17,12z M20,17.5h-2.5V20h-1v-2.5H14v-1h2.5V14h1v2.5H20V17.5z"/>
        </SvgIcon>
    }
}
