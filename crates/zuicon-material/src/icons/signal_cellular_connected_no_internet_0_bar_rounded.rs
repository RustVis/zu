// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularConnectedNoInternet0BarRounded)]
pub fn signal_cellular_connected_no_internet_0_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularConnectedNoInternet0BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,18L21,18c0.55,0,1-0.45,1-1v-6c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v6C20,17.55,20.45,18,21,18z M21,22L21,22 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C20,21.55,20.45,22,21,22z M18,20v2H2L22,2v6h-2V6.83L6.83,20H18z"/>
        </SvgIcon>
    }
}
