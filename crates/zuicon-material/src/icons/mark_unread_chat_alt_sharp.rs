// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkUnreadChatAltSharp)]
pub fn mark_unread_chat_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkUnreadChatAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,8V6h9.03c-1.21-1.6-1.08-3.21-0.92-4H2.01L2,22l4-4h16V6.97C21.16,7.61,20.13,8,19,8H6z M14,14H6v-2h8V14z M18,11H6V9 h12V11z"/>
        </SvgIcon>
    }
}
