// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SolarPowerTwoTone)]
pub fn solar_power_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SolarPowerTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,12H4L2,22h20L20,12z M13,14h5.36l0.4,2H13V14z M11,20H4.44l0.4-2H11V20z M11,16H5.24l0.4-2H11V16z M13,20v-2h6.16 l0.4,2H13z"/><path d="M12,7c2.76,0,5-2.24,5-5h-2c0,1.65-1.35,3-3,3S9,3.65,9,2H7C7,4.76,9.24,7,12,7z"/><path d="M15,2c0,1.66-1.34,3-3,3S9,3.66,9,2H15z" opacity=".3"/>
        </SvgIcon>
    }
}
