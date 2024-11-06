// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PaymentSharp)]
pub fn payment_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PaymentSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 4H2v16h20V4zm-2 14H4v-6h16v6zm0-10H4V6h16v2z"/>
        </SvgIcon>
    }
}
