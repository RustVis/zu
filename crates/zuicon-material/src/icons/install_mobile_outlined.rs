// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InstallMobileOutlined)]
pub fn install_mobile_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InstallMobileOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,18H7V6h7V4H7V3h7V1.01L7,1C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-5h-2V18z M17,21H7v-1h10V21z"/>
        </SvgIcon>
    }
}
