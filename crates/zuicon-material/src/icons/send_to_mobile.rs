// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SendToMobile)]
pub fn send_to_mobile(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SendToMobile"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,17h2v4c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V3c0-1.1,0.9-1.99,2-1.99L17,1c1.1,0,2,0.9,2,2v4h-2V6H7v12h10V17z M22,12 l-4-4v3h-5v2h5v3L22,12z"/>
        </SvgIcon>
    }
}
