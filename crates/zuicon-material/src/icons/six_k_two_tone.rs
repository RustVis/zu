// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SixKTwoTone)]
pub fn six_k_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SixKTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14V5H5V19z M13,9h1.5v2.25L16.25,9H18l-2.25,3L18,15h-1.75l-1.75-2.25 V15H13V9z M6.5,10c0-0.55,0.45-1,1-1H11v1.5H8v1h2c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1H7.5c-0.55,0-1-0.45-1-1V10z" enable-background="new" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M7.5,15H10c0.55,0,1-0.45,1-1v-1.5c0-0.55-0.45-1-1-1H8v-1h3V9H7.5c-0.55,0-1,0.45-1,1v4C6.5,14.55,6.95,15,7.5,15z M8,12.5h1.5V14H8V12.5z"/>
        </SvgIcon>
    }
}
