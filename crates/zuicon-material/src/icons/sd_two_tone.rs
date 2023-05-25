// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SdTwoTone)]
pub fn sd_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SdTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18h16V6H4V18z M13,9h4c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1h-4V9z M6,13h1.5v0.5h2v-1H7 c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v1H9.5v-0.5h-2v1H10c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1H7 c-0.55,0-1-0.45-1-1V13z" opacity=".3"/><path d="M7,15h3c0.55,0,1-0.45,1-1v-1.5c0-0.55-0.45-1-1-1H7.5v-1h2V11H11v-1c0-0.55-0.45-1-1-1H7c-0.55,0-1,0.45-1,1v1.5 c0,0.55,0.45,1,1,1h2.5v1h-2V13H6v1C6,14.55,6.45,15,7,15z"/><path d="M18,14v-4c0-0.55-0.45-1-1-1h-4v6h4C17.55,15,18,14.55,18,14z M16.5,13.5h-2v-3h2V13.5z"/><path d="M20,4H4C2.89,4,2,4.9,2,6v12c0,1.1,0.89,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M20,18H4V6h16V18z"/>
        </SvgIcon>
    }
}
