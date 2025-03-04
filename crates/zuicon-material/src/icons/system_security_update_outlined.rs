// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SystemSecurityUpdateOutlined)]
pub fn system_security_update_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SystemSecurityUpdateOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17,1.01L7,1C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1.01,17,1.01z M17,21H7v-1h10V21z M17,18H7V6h10V18z M7,4V3h10v1H7z M16,12l-4,4l-4-4l1.41-1.41L11,12.17V8h2v4.17l1.59-1.59L16,12z"/>
        </SvgIcon>
    }
}
