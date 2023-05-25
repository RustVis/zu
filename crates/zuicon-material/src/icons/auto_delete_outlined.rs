// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoDeleteOutlined)]
pub fn auto_delete_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AutoDeleteOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M16,9c-0.7,0-1.37,0.1-2,0.29V5H2v12c0,1.1,0.9,2,2,2h5.68c1.12,2.36,3.53,4,6.32,4c3.87,0,7-3.13,7-7 C23,12.13,19.87,9,16,9z M9,16c0,0.34,0.03,0.67,0.08,1H4V7h8v3.26C10.19,11.53,9,13.62,9,16z M16,21c-2.76,0-5-2.24-5-5 s2.24-5,5-5s5,2.24,5,5S18.76,21,16,21z"/>
        </SvgIcon>
    }
}
