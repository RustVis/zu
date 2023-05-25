// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddHomeRounded)]
pub fn add_home_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AddHomeRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M16.53,11.16c1.23-0.26,2.4-0.18,3.47,0.14V10c0-0.63-0.3-1.22-0.8-1.6l-6-4.5c-0.71-0.53-1.69-0.53-2.4,0l-6,4.5 C4.3,8.78,4,9.37,4,10v9c0,1.1,0.9,2,2,2h5.68c-0.61-1.28-0.86-2.77-0.55-4.35C11.65,13.93,13.82,11.74,16.53,11.16z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M21,18.5h-2.5V21h-1v-2.5H15v-1h2.5V15h1v2.5H21V18.5z"/>
        </SvgIcon>
    }
}
