// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddHomeOutlined)]
pub fn add_home_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AddHomeOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M12,5.5l6,4.5v1c0.7,0,1.37,0.1,2,0.29V9l-8-6L4,9v12h7.68c-0.3-0.62-0.5-1.29-0.6-2H6v-9L12,5.5z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M21,18.5h-2.5V21h-1v-2.5H15v-1h2.5V15h1v2.5H21V18.5z"/>
        </SvgIcon>
    }
}
