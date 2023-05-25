// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PowerTwoTone)]
pub fn power_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PowerTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 13.65l3.5 3.52V19h1v-1.83l3.5-3.51V9H8z" opacity=".3"/><path d="M16 7V3h-2v4h-4V3H8v4h-.01C6.89 7 6 7.89 6 8.98v5.52L9.5 18v3h5v-3l3.5-3.5V9c0-1.1-.9-2-2-2zm0 6.66l-3.5 3.51V19h-1v-1.83L8 13.65V9h8v4.66z"/>
        </SvgIcon>
    }
}
