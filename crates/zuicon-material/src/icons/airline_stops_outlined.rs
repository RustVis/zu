// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlineStopsOutlined)]
pub fn airline_stops_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlineStopsOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,8.7c-2.46,1.5-5.5,4.17-6,8.3h2v2H9v-2h2c-0.5-4.5-4.37-8-9-8V7c4.39,0,8.22,2.55,10,6.3c1.38-2.97,3.86-5.03,5.96-6.31 L14,7V5h7v7h-2V8.7z"/>
        </SvgIcon>
    }
}
