// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SolarPower)]
pub fn solar_power(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SolarPower"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,7c2.76,0,5-2.24,5-5H7C7,4.76,9.24,7,12,7z"/>
        </SvgIcon>
    }
}
