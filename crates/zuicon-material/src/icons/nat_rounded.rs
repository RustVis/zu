// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NatRounded)]
pub fn nat_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NatRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M6.82,13H11v-2H6.82C6.4,9.84,5.3,9,4,9c-1.66,0-3,1.34-3,3s1.34,3,3,3C5.3,15,6.4,14.16,6.82,13z M4,13 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C5,12.55,4.55,13,4,13z"/><path d="M22.47,12.4c0.27-0.2,0.27-0.6,0-0.8L19,9v2h-4.05c-0.47-4.69-4.16-8.42-8.83-8.94C5.52,2,5,2.46,5,3.06v0 c0,0.5,0.37,0.93,0.87,0.99C9.88,4.48,13,7.87,13,12s-3.12,7.52-7.13,7.95C5.37,20.01,5,20.44,5,20.94v0c0,0.6,0.52,1.07,1.11,1 c4.67-0.52,8.37-4.25,8.83-8.94H19v2L22.47,12.4z"/>
        </SvgIcon>
    }
}
