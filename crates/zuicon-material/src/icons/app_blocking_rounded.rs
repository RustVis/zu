// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AppBlockingRounded)]
pub fn app_blocking_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AppBlockingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,8c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C22,9.79,20.21,8,18,8z M15.5,12c0-1.38,1.12-2.5,2.5-2.5 c0.42,0,0.8,0.11,1.15,0.29l-3.36,3.36C15.61,12.8,15.5,12.42,15.5,12z M18,14.5c-0.42,0-0.8-0.11-1.15-0.29l3.36-3.36 c0.18,0.35,0.29,0.73,0.29,1.15C20.5,13.38,19.38,14.5,18,14.5z"/><path d="M17,18H7V6h10v1h2V6V5V3c0-1.1-0.9-2-2-2H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-2v-1v-1h-2V18z"/>
        </SvgIcon>
    }
}
