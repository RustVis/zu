// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsCarFilledRounded)]
pub fn directions_car_filled_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsCarFilledRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.92,6.01C18.72,5.42,18.16,5,17.5,5h-11C5.84,5,5.29,5.42,5.08,6.01L3,12v7.5C3,20.33,3.67,21,4.5,21h0 C5.33,21,6,20.33,6,19.5V19h12v0.5c0,0.82,0.67,1.5,1.5,1.5h0c0.82,0,1.5-0.67,1.5-1.5V12L18.92,6.01z M7.5,16 C6.67,16,6,15.33,6,14.5S6.67,13,7.5,13S9,13.67,9,14.5S8.33,16,7.5,16z M16.5,16c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5S17.33,16,16.5,16z M5.81,10l1.04-3h10.29l1.04,3H5.81z"/>
        </SvgIcon>
    }
}
