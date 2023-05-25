// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Grid3x3Rounded)]
pub fn grid_3x3_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Grid3x3Rounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,9L20,9c0-0.55-0.45-1-1-1h-3V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3h-4V5c0-0.55-0.45-1-1-1h0C8.45,4,8,4.45,8,5 v3H5C4.45,8,4,8.45,4,9v0c0,0.55,0.45,1,1,1h3v4H5c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h3v3c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-3h4v3c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3h3c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-3v-4h3 C19.55,10,20,9.55,20,9z M14,14h-4v-4h4V14z"/>
        </SvgIcon>
    }
}
