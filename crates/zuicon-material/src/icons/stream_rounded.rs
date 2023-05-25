// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StreamRounded)]
pub fn stream_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="StreamRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M7.89,14.65l-2.94,2.93c-0.39,0.39-0.39,1.02,0,1.41s1.02,0.39,1.41,0l2.94-2.93c0.39-0.38,0.39-1.02,0-1.41 C8.91,14.26,8.28,14.26,7.89,14.65z"/><path d="M6.41,4.94C6.02,4.55,5.39,4.55,5,4.94C4.61,5.33,4.61,5.96,5,6.35l2.93,2.94c0.39,0.39,1.02,0.39,1.42,0 C9.73,8.9,9.73,8.27,9.34,7.88L6.41,4.94z"/><path d="M16.12,14.65c-0.39-0.39-1.02-0.39-1.42,0c-0.39,0.39-0.39,1.02,0,1.41L17.64,19c0.39,0.39,1.02,0.39,1.41,0 s0.39-1.02,0-1.41L16.12,14.65z"/><path d="M16.06,9.33l2.99-2.98c0.39-0.4,0.39-1.03,0-1.42c-0.39-0.39-1.02-0.39-1.41,0l-2.99,2.98c-0.39,0.39-0.39,1.02,0,1.42 C15.04,9.72,15.67,9.72,16.06,9.33z"/>
        </SvgIcon>
    }
}
