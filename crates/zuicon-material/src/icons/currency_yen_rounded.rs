// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurrencyYenRounded)]
pub fn currency_yen_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurrencyYenRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.82,3c0.34,0,0.66,0.17,0.84,0.46L12,10.29l4.34-6.83C16.52,3.17,16.84,3,17.18,3c0.79,0,1.27,0.87,0.84,1.54L13.92,11 H17c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1h-4v2h4c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1h-4v3c0,0.55-0.45,1-1,1s-1-0.45-1-1v-3H7 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h4v-2H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3.08L5.98,4.54C5.55,3.87,6.03,3,6.82,3z"/>
        </SvgIcon>
    }
}
