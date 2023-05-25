// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkWifi3BarTwoTone)]
pub fn network_wifi_3_bar_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkWifi3BarTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.92,9.07C5.51,7.08,8.67,6,12,6s6.49,1.08,9.08,3.07l-2.85,2.86C16.46,10.71,14.31,10,12,10 c-2.31,0-4.46,0.71-6.23,1.93L2.92,9.07z" opacity=".3"/><path d="M12,4C7.31,4,3.07,5.9,0,8.98L12,21L24,8.98C20.93,5.9,16.69,4,12,4z M2.92,9.07C5.51,7.08,8.67,6,12,6 s6.49,1.08,9.08,3.07l-2.85,2.86C16.46,10.71,14.31,10,12,10c-2.31,0-4.46,0.71-6.23,1.93L2.92,9.07z"/>
        </SvgIcon>
    }
}
