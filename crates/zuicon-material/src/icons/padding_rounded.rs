// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PaddingRounded)]
pub fn padding_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PaddingRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5z M9,8c0,0.55-0.45,1-1,1S7,8.55,7,8 s0.45-1,1-1S9,7.45,9,8z M13,8c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S13,7.45,13,8z M17,8c0,0.55-0.45,1-1,1 c-0.55,0-1-0.45-1-1s0.45-1,1-1C16.55,7,17,7.45,17,8z"/>
        </SvgIcon>
    }
}
