// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RepartitionTwoTone)]
pub fn repartition_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RepartitionTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,21h18v-6H3V21z M15.67,17H19v2h-3.33V17z M10.33,17h3.33v2h-3.33V17z M5,17h3.33v2H5V17z"/><path d="M6,10l1.42-1.42L5.83,7H17c1.1,0,2,0.9,2,2s-0.9,2-2,2H3v2h14c2.21,0,4-1.79,4-4s-1.79-4-4-4H5.83l1.59-1.59L6,2L2,6L6,10 z"/>
        </SvgIcon>
    }
}
