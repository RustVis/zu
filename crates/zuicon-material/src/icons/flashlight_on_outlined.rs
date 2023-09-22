// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlashlightOnOutlined)]
pub fn flashlight_on_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlashlightOnOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,2H6v6l2,3v11h8V11l2-3V2z M16,4v1H8V4H16z M14,10.4V20h-4v-9.61l-2-3V7h8v0.39L14,10.4z"/>
        </SvgIcon>
    }
}
