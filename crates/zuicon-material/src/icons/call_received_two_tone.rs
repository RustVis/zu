// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CallReceivedTwoTone)]
pub fn call_received_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CallReceivedTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M15 17H8.41L20 5.41 18.59 4 7 15.59V9H5v10h10z"/>
        </SvgIcon>
    }
}
