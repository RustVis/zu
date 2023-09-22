// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SubscriptSharp)]
pub fn subscript_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SubscriptSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,18v1h3v1h-4v-3h3v-1h-3v-1h4v3H20z M5.88,18h2.66l3.4-5.42h0.12l3.4,5.42h2.66l-4.65-7.27L17.81,4h-2.68l-3.07,4.99 h-0.12L8.85,4H6.19l4.32,6.73L5.88,18z"/>
        </SvgIcon>
    }
}
