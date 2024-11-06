// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CoffeeMakerRounded)]
pub fn coffee_maker_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CoffeeMakerRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,6V4h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h13c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-3.03C17.2,19.09,18,17.64,18,16v-3c0-1.1-0.9-2-2-2h-6c-1.1,0-2,0.9-2,2v3c0,1.64,0.81,3.09,2.03,4H6V4h2v2 c0,0.55,0.45,1,1,1h8C17.55,7,18,6.55,18,6z"/>
        </SvgIcon>
    }
}
