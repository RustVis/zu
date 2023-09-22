// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FireTruckOutlined)]
pub fn fire_truck_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FireTruckOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.9,10.69l-1.44-4.32C21.18,5.55,20.42,5,19.56,5H19V4c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v1h-2c-1.1,0-2,0.9-2,2 v4H1v5c0,1.1,0.9,2,2,2h1c0,1.66,1.34,3,3,3s3-1.34,3-3h4c0,1.66,1.34,3,3,3s3-1.34,3-3h3v-6.68C23,11.11,22.97,10.9,22.9,10.69z M14,7h5.56l1.33,4H14V7z M7,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S7.55,19,7,19z M12,16H9.22C8.67,15.39,7.89,15,7,15 s-1.67,0.39-2.22,1H3v-3h9V16z M17,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S17.55,19,17,19z M19.22,16 c-0.55-0.61-1.34-1-2.22-1s-1.67,0.39-2.22,1H14v-3h7v3H19.22z"/><path d="M11,8.5h-1v-2h1V5H1v1.5h1v2H1V10h10V8.5z M8.5,8.5H6.75v-2H8.5V8.5z M3.5,6.5h1.75v2H3.5V6.5z"/>
        </SvgIcon>
    }
}
