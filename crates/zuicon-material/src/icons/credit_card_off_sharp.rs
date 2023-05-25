// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CreditCardOffSharp)]
pub fn credit_card_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CreditCardOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.83,4H22v15.17L14.83,12H20V8h-9.17L6.83,4z M20.49,23.31L17.17,20H2V4.83L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M9.17,12l-4-4H4v4H9.17z"/>
        </SvgIcon>
    }
}
