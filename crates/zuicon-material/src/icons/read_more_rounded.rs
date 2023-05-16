// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ReadMoreRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ReadMoreRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M14,9h7c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-7c-0.55,0-1,0.45-1,1C13,8.55,13.45,9,14,9z"/><path d="M21,15h-7c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h7c0.55,0,1-0.45,1-1C22,15.45,21.55,15,21,15z"/><path d="M21,11h-4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1C22,11.45,21.55,11,21,11z"/><path d="M8.85,7.85C8.54,7.54,8,7.76,8,8.21V11H3c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h5v2.79c0,0.45,0.54,0.67,0.85,0.35 l3.79-3.79c0.2-0.2,0.2-0.51,0-0.71L8.85,7.85z"/>
        </SvgIcon>
    }
}
