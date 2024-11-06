// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnSlightLeft)]
pub fn turn_slight_left(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnSlightLeft"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.66,6V4H6v5.66h2V7.41l5,5V20h2v-7.58c0-0.53-0.21-1.04-0.59-1.41l-5-5H11.66z"/>
        </SvgIcon>
    }
}
