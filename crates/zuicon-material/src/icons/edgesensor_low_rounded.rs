// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EdgesensorLowRounded)]
pub fn edgesensor_low_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EdgesensorLowRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M3,7L3,7c0.55,0,1,0.45,1,1v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V8C2,7.45,2.45,7,3,7z M21,10L21,10 c0.55,0,1,0.45,1,1v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-5C20,10.45,20.45,10,21,10z M16,2.01L8,2C6.9,2,6,2.9,6,4v16 c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V4C18,2.9,17.1,2.01,16,2.01z M16,17H8V7h8V17z"/>
        </SvgIcon>
    }
}
