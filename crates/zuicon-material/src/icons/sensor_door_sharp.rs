// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SensorDoorSharp)]
pub fn sensor_door_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SensorDoorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2H4v20h16V2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5S17,11.17,17,12S16.33,13.5,15.5,13.5z"/>
        </SvgIcon>
    }
}
