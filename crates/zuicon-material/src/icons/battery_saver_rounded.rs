// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BatterySaverRounded)]
pub fn battery_saver_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BatterySaverRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,4h-2V3c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v1H8C7.45,4,7,4.45,7,5v16c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1V5 C17,4.45,16.55,4,16,4z M14,14h-1v1c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-1h-1c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h1v-1 c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v1h1c0.55,0,1,0.45,1,1v0C15,13.55,14.55,14,14,14z"/>
        </SvgIcon>
    }
}
