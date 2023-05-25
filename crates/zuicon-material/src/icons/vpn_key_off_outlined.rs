// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VpnKeyOffOutlined)]
pub fn vpn_key_off_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VpnKeyOffOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.81,2.81L1.39,4.22l2.59,2.59C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6c2.22,0,4.15-1.21,5.19-3l7.59,7.61l1.41-1.41 L2.81,2.81z M7,16c-2.21,0-4-1.79-4-4c0-1.67,1.02-3.1,2.47-3.7l1.71,1.71C7.12,10,7.06,10,7,10c-1.1,0-2,0.9-2,2s0.9,2,2,2 s2-0.9,2-2c0-0.06,0-0.12-0.01-0.18l1.74,1.74C10.22,14.48,9.14,16,7,16z M17,14.17V13h-1.17L17,14.17z M13.83,11H21v2h-2v3l2,2 v-3h2V9H11.83L13.83,11z"/>
        </SvgIcon>
    }
}
