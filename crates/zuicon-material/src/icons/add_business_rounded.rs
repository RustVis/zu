// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddBusinessRounded)]
pub fn add_business_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddBusinessRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,6h13c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H3C2.45,4,2,4.45,2,5C2,5.55,2.45,6,3,6z"/><path d="M15,17h2v-3h0.18c0.63,0,1.1-0.58,0.98-1.2l-1-5C17.07,7.34,16.66,7,16.18,7H2.82C2.34,7,1.93,7.34,1.84,7.8l-1,5 C0.72,13.42,1.19,14,1.82,14H2v5c0,0.55,0.45,1,1,1h7c0.55,0,1-0.45,1-1v-5h4V17z M9,18H4v-4h5V18z"/><path d="M22,18h-2v-2c0-0.55-0.45-1-1-1s-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1 v-2h2c0.55,0,1-0.45,1-1C23,18.45,22.55,18,22,18z"/>
        </SvgIcon>
    }
}
