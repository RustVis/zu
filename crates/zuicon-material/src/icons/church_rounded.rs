// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChurchRounded)]
pub fn church_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ChurchRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M18,12.22v-1.99c0-0.76-0.43-1.45-1.11-1.79L13,6.5V5h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1V2c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v1h-1C9.45,3,9,3.45,9,4v0c0,0.55,0.45,1,1,1h1v1.5L7.11,8.45C6.43,8.79,6,9.48,6,10.24v1.99l-2.81,1.25 C2.47,13.79,2,14.51,2,15.3V20c0,1.1,0.9,2,2,2h6l0-2.89c0-1,0.68-1.92,1.66-2.08C12.92,16.82,14,17.79,14,19v3h6c1.1,0,2-0.9,2-2 v-4.7c0-0.79-0.47-1.51-1.19-1.83L18,12.22z M12,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S12.83,13.5,12,13.5z"/>
        </SvgIcon>
    }
}
