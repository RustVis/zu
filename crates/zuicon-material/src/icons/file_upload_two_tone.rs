// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FileUploadTwoTone)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="FileUploadTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M5,10h4v6h6v-6h4l-7-7L5,10z M13,8v6h-2V8H9.83L12,5.83L14.17,8H13z"/>
        </SvgIcon>
    }
}
