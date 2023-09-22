// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoChatSharp)]
pub fn video_chat_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoChatSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,2v20l4-4h16V2H2z M17,13l-2-1.99V14H7V6h8v2.99L17,7V13z"/>
        </SvgIcon>
    }
}
