// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KeyOff)]
pub fn key_off(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KeyOff"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.91,14.09L17,14l2,2l4-4.04L21,10h-8.17L16.91,14.09z M3.98,6.81C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6 c2.21,0,4.15-1.2,5.18-2.99l7.59,7.59l1.41-1.41L2.81,2.81L1.39,4.22L3.98,6.81z M9.91,12.74C9.58,14.03,8.4,15,7,15 c-1.65,0-3-1.35-3-3c0-1.4,0.97-2.58,2.26-2.91L9.91,12.74z"/>
        </SvgIcon>
    }
}
