// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CallReceived)]
pub fn call_received(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CallReceived"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M20 5.41L18.59 4 7 15.59V9H5v10h10v-2H8.41z"/>
        </SvgIcon>
    }
}
