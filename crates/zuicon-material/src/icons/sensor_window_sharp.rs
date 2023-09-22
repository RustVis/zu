// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SensorWindowSharp)]
pub fn sensor_window_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SensorWindowSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,4v16H6V4H18 M4,2v20h16V2H4z M7,19h10v-6H7V19z M10,10h4v1h3V5H7v6h3V10z"/>
        </SvgIcon>
    }
}
