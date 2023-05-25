// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VpnKeyOffRounded)]
pub fn vpn_key_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VpnKeyOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.98,6.81C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6c2.21,0,4.15-1.2,5.18-2.99l6.89,6.89c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L3.98,6.81z M8.99,11.82 C9,11.88,9,11.94,9,12c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2c0.06,0,0.12,0,0.18,0.01L8.99,11.82z M20.32,17.5 C20.74,17.13,21,16.59,21,16v-2c1.1,0,2-0.9,2-2s-0.9-2-2-2h-8.17L20.32,17.5"/>
        </SvgIcon>
    }
}
