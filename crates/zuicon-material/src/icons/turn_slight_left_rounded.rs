// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnSlightLeftRounded)]
pub fn turn_slight_left_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnSlightLeftRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.66,5L11.66,5c0-0.55-0.45-1-1-1H7C6.45,4,6,4.45,6,5v3.66c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V7.41l5,5V19 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6.58c0-0.53-0.21-1.04-0.59-1.41l-5-5h1.24C11.21,6,11.66,5.55,11.66,5z"/>
        </SvgIcon>
    }
}
