// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkWifiOutlined)]
pub fn network_wifi_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkWifiOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,4C7.31,4,3.07,5.9,0,8.98L12,21L24,8.98C20.93,5.9,16.69,4,12,4z M12,8c-2.86,0-5.5,0.94-7.65,2.51L2.92,9.07 C5.51,7.08,8.67,6,12,6s6.49,1.08,9.08,3.07l-1.43,1.43C17.5,8.94,14.86,8,12,8z"/>
        </SvgIcon>
    }
}
