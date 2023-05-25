// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardDoubleArrowUpRounded)]
pub fn keyboard_double_arrow_up_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KeyboardDoubleArrowUpRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.7,18.29L6.7,18.29c0.39,0.39,1.02,0.39,1.41,0L12,14.42l3.88,3.88c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41l-4.59-4.59c-0.39-0.39-1.02-0.39-1.41,0L6.7,16.88C6.31,17.27,6.31,17.9,6.7,18.29z"/><path d="M6.7,11.7L6.7,11.7c0.39,0.39,1.02,0.39,1.41,0L12,7.83l3.88,3.88c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41l-4.59-4.59c-0.39-0.39-1.02-0.39-1.41,0L6.7,10.29C6.31,10.68,6.31,11.31,6.7,11.7z"/>
        </SvgIcon>
    }
}
