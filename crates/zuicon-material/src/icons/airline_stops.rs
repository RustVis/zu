// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlineStops)]
pub fn airline_stops(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlineStops"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.21,9.21C15.93,10.78,13.45,13.3,13,17h2v2H9v-2h2c-0.5-4.5-4.37-8-9-8V7c4.39,0,8.22,2.55,10,6.3 c1.13-2.43,2.99-4.25,4.78-5.52L14,5h7v7L18.21,9.21z"/>
        </SvgIcon>
    }
}
