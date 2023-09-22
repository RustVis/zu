// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalWifiStatusbarConnectedNoInternet4Rounded)]
pub fn signal_wifi_statusbar_connected_no_internet_4_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalWifiStatusbarConnectedNoInternet4Rounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.92,8H17v7.99l-4.29,4.3c-0.39,0.39-1.02,0.39-1.42,0L0.73,9.71C0.32,9.3,0.35,8.63,0.79,8.24C3.78,5.6,7.7,4,12,4 C16.16,4,19.97,5.51,22.92,8z M20,18c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S20.55,18,20,18z M20,10c-0.55,0-1,0.45-1,1v4 c0,0.55,0.45,1,1,1s1-0.45,1-1v-4C21,10.45,20.55,10,20,10z"/>
        </SvgIcon>
    }
}
