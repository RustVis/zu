// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FastForwardTwoTone)]
pub fn fast_forward_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FastForwardTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18l8.5-6L4,6V18z M6,9.86L9.03,12L6,14.14V9.86z"/><path d="M21.5,12L13,6v12L21.5,12z M15,9.86L18.03,12L15,14.14V9.86z"/>
        </SvgIcon>
    }
}
