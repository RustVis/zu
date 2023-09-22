// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AttachEmailSharp)]
pub fn attach_email_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AttachEmailSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,10V2H1v16h13v-5c0-1.66,1.34-3,3-3H21z M11,11L3,6V4l8,5l8-5v2L11,11z"/><path d="M21,14v4c0,1.1-0.9,2-2,2s-2-0.9-2-2v-4.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V18h2v-4.5c0-1.38-1.12-2.5-2.5-2.5 S15,12.12,15,13.5V18c0,2.21,1.79,4,4,4s4-1.79,4-4v-4H21z"/>
        </SvgIcon>
    }
}
