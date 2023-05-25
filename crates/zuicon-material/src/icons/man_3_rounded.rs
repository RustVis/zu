// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Man3Rounded)]
pub fn man_3_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Man3Rounded"
            view_box={props.view_box.clone()}
            >
            <path d="M14,7h-4C8.9,7,8,7.9,8,9v5c0,0.55,0.45,1,1,1h1v6c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-6h1c0.55,0,1-0.45,1-1V9 C16,7.9,15.1,7,14,7z"/><path d="M11.65,5.9L10.1,4.35c-0.2-0.2-0.2-0.51,0-0.71l1.54-1.54c0.2-0.2,0.51-0.2,0.71,0l1.54,1.54c0.2,0.2,0.2,0.51,0,0.71 L12.35,5.9C12.16,6.09,11.84,6.09,11.65,5.9z"/>
        </SvgIcon>
    }
}
