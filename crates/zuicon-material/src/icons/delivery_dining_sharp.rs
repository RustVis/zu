// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DeliveryDiningSharp)]
pub fn delivery_dining_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DeliveryDiningSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,10.35V5h-5v2h3v2.65L13.52,14H10V9H2v7h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35z M7,17c-0.55,0-1-0.45-1-1h2 C8,16.55,7.55,17,7,17z"/><path d="M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 C20,16.55,19.55,17,19,17z"/>
        </SvgIcon>
    }
}
