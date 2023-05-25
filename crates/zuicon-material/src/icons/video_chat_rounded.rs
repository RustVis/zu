// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoChatRounded)]
pub fn video_chat_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoChatRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2H4C2.9,2,2.01,2.9,2.01,4L2,19.58c0,0.89,1.08,1.34,1.71,0.71L6,18h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M16.15,12.15L15,11.01V13c0,0.55-0.45,1-1,1H8c-0.55,0-1-0.45-1-1V7c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v1.99l1.15-1.14 C16.46,7.53,17,7.76,17,8.2v3.59C17,12.24,16.46,12.47,16.15,12.15z"/>
        </SvgIcon>
    }
}
