// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LtePlusMobiledataRounded)]
pub fn lte_plus_mobiledata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LtePlusMobiledataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M3,14h2c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H2c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1V14z M6,10 h1v5c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-5h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6C5.45,8,5,8.45,5,9v0 C5,9.55,5.45,10,6,10z M13,16h3c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2v-1h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2v-1h2 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v6C12,15.55,12.45,16,13,16z M23,11h-1v-1c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v1h-1c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h1 c0.55,0,1-0.45,1-1v0C24,11.45,23.55,11,23,11z"/>
        </SvgIcon>
    }
}
