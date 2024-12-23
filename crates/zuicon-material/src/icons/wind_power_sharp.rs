// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WindPowerSharp)]
pub fn wind_power_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WindPowerSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.32,12.09l5.42-9.04L17.32,1L12,5.97v4.74c0.31-0.13,0.64-0.21,1-0.21C14.06,10.5,14.96,11.16,15.32,12.09z"/><path d="M10.5,13c0-0.82,0.4-1.54,1.01-2H1v4l7,2l3.44-2.06C10.87,14.48,10.5,13.79,10.5,13z"/><path d="M20.17,23L23,20.17l-3.54-6.36l-3.98-1c0,0.06,0.02,0.12,0.02,0.19c0,1.38-1.12,2.5-2.5,2.5c-0.36,0-0.69-0.08-1-0.21V21 c-1.1,0-2,0.9-2,2h6c0-1.1-0.9-2-2-2v-4.17L20.17,23z"/>
        </SvgIcon>
    }
}
