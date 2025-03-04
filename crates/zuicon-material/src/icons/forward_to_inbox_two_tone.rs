// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForwardToInboxTwoTone)]
pub fn forward_to_inbox_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForwardToInboxTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,18H4V8l8,5l8-5v5l-2,0C15.24,13,13,15.24,13,18L13,18z M12,11L4,6h16L12,11z" opacity=".3"/><path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h9v-2H4V8l8,5l8-5v5h2V6C22,4.9,21.1,4,20,4z M12,11L4,6h16L12,11z M19,15l4,4 l-4,4v-3h-4v-2h4V15z"/>
        </SvgIcon>
    }
}
