// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Carpenter)]
pub fn carpenter(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Carpenter"
            view_box={props.view_box.clone()}
            >
            <path d="M19.73,14.23L7,1.5L3.11,5.39l8.13,11.67c-0.78,0.78-0.78,2.05,0,2.83l1.41,1.41c0.78,0.78,2.05,0.78,2.83,0l4.24-4.24 C20.51,16.28,20.51,15.01,19.73,14.23z M14.07,19.88l-1.41-1.41l4.24-4.24l1.41,1.41L14.07,19.88z"/>
        </SvgIcon>
    }
}
