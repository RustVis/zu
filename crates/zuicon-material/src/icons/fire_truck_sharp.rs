// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FireTruckSharp)]
pub fn fire_truck_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FireTruckSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M23,11l-2-6h-2V3h-3v2h-4v6H1v7h3c0,1.66,1.34,3,3,3s3-1.34,3-3h4c0,1.66,1.34,3,3,3s3-1.34,3-3h3V11z M7,19 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S7.55,19,7,19z M17,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S17.55,19,17,19z M14,11V7h5.56l1.33,4H14z"/><path d="M11,8.5h-1v-2h1V5H1v1.5h1v2H1V10h10V8.5z M5.25,8.5H3.5v-2h1.75V8.5z M8.5,8.5H6.75v-2H8.5V8.5z"/>
        </SvgIcon>
    }
}
