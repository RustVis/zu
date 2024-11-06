// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MonitorWeightSharp)]
pub fn monitor_weight_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MonitorWeightSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21,3H3v18h18V3z M14.2,11.2c-3.23,2.43-6.84-1.18-4.4-4.41C13.03,4.37,16.63,7.98,14.2,11.2z"/>
        </SvgIcon>
    }
}
