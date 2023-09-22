// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignpostTwoTone)]
pub fn signpost_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignpostTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,6h11.17l1,1l-1,1H6V6z M18,16H6.83l-1-1l1-1H18V16z" opacity=".3"/><path d="M13,10h5l3-3l-3-3h-5V2h-2v2H4v6h7v2H6l-3,3l3,3h5v4h2v-4h7v-6h-7V10z M6,6h11.17l1,1l-1,1H6V6z M18,16H6.83l-1-1l1-1H18 V16z"/>
        </SvgIcon>
    }
}
