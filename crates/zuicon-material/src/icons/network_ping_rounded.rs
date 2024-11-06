// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkPingRounded)]
pub fn network_ping_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkPingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.71,6.79c-0.39,0.39-0.39,1.02,0,1.41L10.5,16H5c-0.55,0-1,0.45-1,1s0.45,1,1,1h14c0.55,0,1-0.45,1-1s-0.45-1-1-1h-5.5 l5.15-5.15C18.91,10.95,19.2,11,19.5,11c1.38,0,2.5-1.12,2.5-2.5S20.88,6,19.5,6S17,7.12,17,8.5c0,0.35,0.07,0.67,0.2,0.97 l-5.2,5.2L4.12,6.79C3.73,6.4,3.1,6.4,2.71,6.79z"/>
        </SvgIcon>
    }
}
