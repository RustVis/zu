// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularConnectedNoInternet4BarRounded)]
pub fn signal_cellular_connected_no_internet_4_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularConnectedNoInternet4BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21,18L21,18c0.55,0,1-0.45,1-1v-6c0-0.55-0.45-1-1-1l0,0c-0.55,0-1,0.45-1,1v6C20,17.55,20.45,18,21,18z M21,22L21,22 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C20,21.55,20.45,22,21,22z M4.41,22H18V11c0-1.66,1.34-3,3-3h1 V4.41c0-0.89-1.08-1.34-1.71-0.71L3.71,20.29C3.08,20.92,3.52,22,4.41,22z"/>
        </SvgIcon>
    }
}
