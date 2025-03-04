// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElectricalServicesRounded)]
pub fn electrical_services_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ElectricalServicesRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,14c0-0.55-0.45-1-1-1h-2v2h2C20.55,15,21,14.55,21,14z"/><path d="M20,17h-2v2h2c0.55,0,1-0.45,1-1C21,17.45,20.55,17,20,17z"/><path d="M16,12h-2c-1.1,0-2,0.9-2,2h-1c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1h1c0,1.1,0.9,2,2,2h2c0.55,0,1-0.45,1-1v-6 C17,12.45,16.55,12,16,12z"/><path d="M5,13c0-1.1,0.9-2,2-2h1.5c1.93,0,3.5-1.57,3.5-3.5S10.43,4,8.5,4H5C4.45,4,4,4.45,4,5c0,0.55,0.45,1,1,1h3.5 C9.33,6,10,6.67,10,7.5S9.33,9,8.5,9H7c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4h2v-2H7C5.9,15,5,14.1,5,13z"/>
        </SvgIcon>
    }
}
