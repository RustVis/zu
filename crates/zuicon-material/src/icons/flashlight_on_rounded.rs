// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlashlightOnRounded)]
pub fn flashlight_on_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlashlightOnRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M6,4v1h12V4c0-1.1-0.9-2-2-2H8C6.9,2,6,2.9,6,4z"/><path d="M6,7v1l2,3v9c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2v-9l2-3V7H6z M12,15.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5S12.83,15.5,12,15.5z"/>
        </SvgIcon>
    }
}
