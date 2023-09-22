// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChairAltRounded)]
pub fn chair_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChairAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17,10c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v3c0,1.1,0.9,2,2,2h1v2H7c-1.1,0-2,0.9-2,2v6 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h10v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6c0-1.1-0.9-2-2-2h-1v-2H17z M7,8V5h10v3 H7z M17,16H7v-2h10V16z M14,12h-4v-2h4V12z"/>
        </SvgIcon>
    }
}
