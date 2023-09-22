// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularConnectedNoInternet0BarOutlined)]
pub fn signal_cellular_connected_no_internet_0_bar_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularConnectedNoInternet0BarOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,18h2v-8h-2V18z M20,22h2v-2h-2V22z M18,20v2H2L22,2v6h-2V6.83L6.83,20H18z"/>
        </SvgIcon>
    }
}
