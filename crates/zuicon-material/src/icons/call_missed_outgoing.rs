// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CallMissedOutgoing)]
pub fn call_missed_outgoing(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CallMissedOutgoing"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,8.41l9,9l7-7V15h2V7h-8v2h4.59L12,14.59L4.41,7L3,8.41z"/>
        </SvgIcon>
    }
}
