// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShuffleOnSharp)]
pub fn shuffle_on_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShuffleOnSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1,1v22h22V1H1z M5.41,4l5.18,5.17l-1.42,1.41L4,5.41L5.41,4z M20,20h-5.5l2.05-2.05l-3.13-3.13l1.41-1.41l3.13,3.13 L20,14.5V20z M20,9.5l-2.04-2.04L5.41,20L4,18.59L16.54,6.04L14.5,4H20V9.5z"/>
        </SvgIcon>
    }
}
