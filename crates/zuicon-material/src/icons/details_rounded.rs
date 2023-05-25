// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DetailsRounded)]
pub fn details_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="DetailsRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M11.13,4.57l-8.3,14.94C2.46,20.18,2.94,21,3.7,21h16.6c0.76,0,1.24-0.82,0.87-1.49l-8.3-14.94 C12.49,3.89,11.51,3.89,11.13,4.57z M13,8.92L18.6,19H13V8.92z M11,8.92V19H5.4L11,8.92z"/>
        </SvgIcon>
    }
}
