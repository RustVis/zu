// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DiningRounded)]
pub fn dining_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DiningRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M11,10.3c0,0.93-0.64,1.71-1.5,1.93 v6.02C9.5,18.66,9.16,19,8.75,19h0C8.34,19,8,18.66,8,18.25v-6.02c-0.86-0.22-1.5-1-1.5-1.93V6.5C6.5,6.22,6.72,6,7,6 s0.5,0.22,0.5,0.5V9h0.75V6.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V9H10V6.5C10,6.22,10.23,6,10.5,6C10.78,6,11,6.22,11,6.5 V10.3z M15.58,12.59l-0.08,0.03v5.63c0,0.41-0.34,0.75-0.75,0.75h0C14.34,19,14,18.66,14,18.25v-5.63l-0.08-0.04 c-0.97-0.47-1.67-1.7-1.67-3.18c0-1.88,1.13-3.4,2.5-3.4c1.38,0,2.5,1.53,2.5,3.41C17.25,10.89,16.55,12.12,15.58,12.59z"/>
        </SvgIcon>
    }
}
