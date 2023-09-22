// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkUnreadChatAltOutlined)]
pub fn mark_unread_chat_alt_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkUnreadChatAltOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,16H4V4h10.1c-0.08-0.39-0.18-1.11,0-2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V6.98c-0.58,0.44-1.26,0.77-2,0.92 V16z"/><path d="M6,8h12V7.9c-1.21-0.25-2.25-0.95-2.97-1.9H6V8z"/>
        </SvgIcon>
    }
}
