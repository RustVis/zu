// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlashlightOnTwoTone)]
pub fn flashlight_on_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlashlightOnTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M8,7.39l2,3V20h4v-9.6l2-3.01V7H8V7.39z M12,12.5c0.83,0,1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5 s-1.5-0.67-1.5-1.5S11.17,12.5,12,12.5z" opacity=".3"/><path d="M6,2v6l2,3v11h8V11l2-3V2H6z M16,7.39l-2,3.01V20h-4v-9.61l-2-3V7h8V7.39z M16,5H8V4h8V5z"/>
        </SvgIcon>
    }
}
