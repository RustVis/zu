// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CreditCardOffTwoTone)]
pub fn credit_card_off_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CreditCardOffTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,17.17V12h-5.17L20,17.17z M10.83,8H20V6H8.83L10.83,8z M4,6.83V8h1.17 L4,6.83z M15.17,18l-6-6H4v6H15.17z" enable-background="new" opacity=".3"/><path d="M6.83,4H20c1.11,0,2,0.89,2,2v12c0,0.34-0.08,0.66-0.23,0.94L20,17.17V12h-5.17l-4-4H20V6H8.83 L6.83,4z M20.49,23.31L17.17,20H4c-1.11,0-2-0.89-2-2L2.01,6c0-0.34,0.08-0.66,0.23-0.93L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M4,6.83V8h1.17L4,6.83z M15.17,18l-6-6H4v6H15.17z" enable-background="new"/>
        </SvgIcon>
    }
}
