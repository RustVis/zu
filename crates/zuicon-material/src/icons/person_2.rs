// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Person2)]
pub fn person_2(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Person2"
            view_box={props.view_box.clone()}
            >
            <path d="M18.39,14.56C16.71,13.7,14.53,13,12,13c-2.53,0-4.71,0.7-6.39,1.56C4.61,15.07,4,16.1,4,17.22V20h16v-2.78 C20,16.1,19.39,15.07,18.39,14.56z"/><path d="M9.78,12h4.44c1.21,0,2.14-1.06,1.98-2.26l-0.32-2.45C15.57,5.39,13.92,4,12,4S8.43,5.39,8.12,7.29L7.8,9.74 C7.64,10.94,8.57,12,9.78,12z"/>
        </SvgIcon>
    }
}
