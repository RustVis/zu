// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkWifi3BarRounded)]
pub fn network_wifi_3_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkWifi3BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4C7.7,4,3.78,5.6,0.79,8.24C0.35,8.63,0.32,9.3,0.73,9.71l10.56,10.58c0.39,0.39,1.02,0.39,1.42,0L23.27,9.71 c0.41-0.41,0.38-1.08-0.06-1.47C20.22,5.6,16.3,4,12,4z M2.92,9.07C5.51,7.08,8.67,6,12,6s6.49,1.08,9.08,3.07l-2.85,2.86 C16.46,10.71,14.31,10,12,10c-2.31,0-4.46,0.71-6.23,1.93L2.92,9.07z"/>
        </SvgIcon>
    }
}
