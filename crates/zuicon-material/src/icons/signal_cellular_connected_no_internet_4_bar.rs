// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularConnectedNoInternet4Bar)]
pub fn signal_cellular_connected_no_internet_4_bar(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularConnectedNoInternet4Bar"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,18h2v-8h-2V18z M20,22h2v-2h-2V22z M2,22h16V8h4V2L2,22z"/>
        </SvgIcon>
    }
}
