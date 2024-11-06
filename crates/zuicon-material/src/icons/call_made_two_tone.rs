// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CallMadeTwoTone)]
pub fn call_made_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CallMadeTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5.41 20L17 8.41V15h2V5H9v2h6.59L4 18.59z"/>
        </SvgIcon>
    }
}
