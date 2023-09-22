// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkUnreadChatAltRounded)]
pub fn mark_unread_chat_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkUnreadChatAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,8C6.45,8,6,7.55,6,7c0-0.55,0.45-1,1-1h8.03c-1.21-1.6-1.08-3.21-0.92-4H4.01c-1.1,0-2,0.89-2,2L2,19.58 c0,0.89,1.08,1.34,1.71,0.71L6,18h14c1.1,0,2-0.9,2-2V6.97C21.16,7.61,20.13,8,19,8H7z M13,14H7c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1C14,13.55,13.55,14,13,14z M17,11H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h10 c0.55,0,1,0.45,1,1C18,10.55,17.55,11,17,11z"/>
        </SvgIcon>
    }
}
