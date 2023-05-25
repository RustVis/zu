// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SensorDoor)]
pub fn sensor_door(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SensorDoor"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 S17,11.17,17,12S16.33,13.5,15.5,13.5z"/>
        </SvgIcon>
    }
}
