// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LocalHospitalTwoTone)]
pub fn local_hospital_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LocalHospitalTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5 19h14V5H5v14zm2-8.5h3.5V7h3v3.5H17v3h-3.5V17h-3v-3.5H7v-3z" opacity=".3"/><path d="M21 5c0-1.1-.9-2-2-2H5c-1.1 0-1.99.9-1.99 2L3 19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5zm-2 14H5V5h14v14zm-8.5-2h3v-3.5H17v-3h-3.5V7h-3v3.5H7v3h3.5z"/>
        </SvgIcon>
    }
}
