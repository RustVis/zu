// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkChatReadTwoTone)]
pub fn mark_chat_read_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkChatReadTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,10c0.34,0,0.67,0.03,1,0.08V4H4v12h8.08C12.57,12.61,15.47,10,19,10z" opacity=".3"/><path d="M17.34,20l-3.54-3.54l1.41-1.41l2.12,2.12l4.24-4.24L23,14.34L17.34,20z M12.08,16H4V4h16v6.08c0.71,0.1,1.38,0.31,2,0.6V4 c0-1.1-0.9-2-2-2H4C2.9,2,2,2.9,2,4v18l4-4h6v0c0-0.14,0.02-0.27,0.03-0.4C12.01,17.4,12,17.2,12,17C12,16.66,12.03,16.33,12.08,16 z"/>
        </SvgIcon>
    }
}
