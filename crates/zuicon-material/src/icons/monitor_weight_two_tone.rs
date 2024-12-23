// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MonitorWeightTwoTone)]
pub fn monitor_weight_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MonitorWeightTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M5,19h14V5H5V19z M12,6c1.66,0,3,1.34,3,3s-1.34,3-3,3s-3-1.34-3-3S10.34,6,12,6z" opacity=".3"/><path d="M12,12c1.66,0,3-1.34,3-3s-1.34-3-3-3S9,7.34,9,9S10.34,12,12,12z M13,8.5h1v1h-1V8.5z M11.5,8.5h1v1h-1V8.5z M10,8.5h1v1 h-1V8.5z"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/>
        </SvgIcon>
    }
}
