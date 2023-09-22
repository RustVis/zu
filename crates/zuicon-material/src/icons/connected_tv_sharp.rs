// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ConnectedTvSharp)]
pub fn connected_tv_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ConnectedTvSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.57,16H10c0-2.76-2.24-5-5-5v1.43C6.97,12.43,8.57,14.03,8.57,16z"/><path d="M11.55,16H13c0-4.42-3.59-8-8-8v1.45C8.61,9.45,11.55,12.38,11.55,16z"/><path d="M5,14v2h2C7,14.89,6.11,14,5,14z"/><path d="M22,3H2v16h6v2h8v-2h6V3z M20,17H4V5h16V17z"/>
        </SvgIcon>
    }
}
