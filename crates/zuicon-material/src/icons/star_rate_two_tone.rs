// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StarRateTwoTone)]
pub fn star_rate_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StarRateTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,10h-7.58L12,2l-2.42,8H2l6.17,4.41L5.83,22L12,17.31L18.17,22l-2.35-7.59L22,10z M14.42,16.63L12,14.79l-2.42,1.84 l0.93-3.01L8.24,12h2.82L12,8.89L12.94,12h2.82l-2.27,1.62L14.42,16.63z"/>
        </SvgIcon>
    }
}
