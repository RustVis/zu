// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CableRounded)]
pub fn cable_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CableRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,5V4c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v1h0c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h1l0,6.91 c0,1.04-0.76,1.98-1.79,2.08C14.01,19.11,13,18.17,13,17l0-9.86c0-2.13-1.61-3.99-3.74-4.13C6.93,2.86,5,4.7,5,7v7H4 c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h0v1c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-1h0c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1 H7l0-6.91c0-1.04,0.76-1.98,1.79-2.08C9.99,4.89,11,5.83,11,7l0,9.86c0,2.13,1.61,3.99,3.74,4.13C17.07,21.14,19,19.3,19,17v-7h1 c0.55,0,1-0.45,1-1V6C21,5.45,20.55,5,20,5L20,5z"/>
        </SvgIcon>
    }
}
