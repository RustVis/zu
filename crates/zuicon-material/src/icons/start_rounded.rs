// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StartRounded)]
pub fn start_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StartRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.29,17.29c0.39,0.39,1.02,0.39,1.41,0l4.59-4.59c0.39-0.39,0.39-1.02,0-1.41L16.7,6.7c-0.39-0.39-1.02-0.39-1.41,0 c-0.38,0.39-0.39,1.03,0,1.42L18.17,11H7c-0.55,0-1,0.45-1,1s0.45,1,1,1h11.17l-2.88,2.88C14.9,16.27,14.9,16.9,15.29,17.29z M3,18 c0.55,0,1-0.45,1-1V7c0-0.55-0.45-1-1-1S2,6.45,2,7v10C2,17.55,2.45,18,3,18z"/>
        </SvgIcon>
    }
}
