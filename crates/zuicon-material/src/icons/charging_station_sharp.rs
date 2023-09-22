// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChargingStationSharp)]
pub fn charging_station_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChargingStationSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.5,11l-3,6v-4h-2l3-6v4H14.5z M5,1h14v22H5V1z M7,6v12h10V6H7z"/>
        </SvgIcon>
    }
}
