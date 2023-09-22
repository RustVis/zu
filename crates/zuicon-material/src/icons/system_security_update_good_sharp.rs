// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SystemSecurityUpdateGoodSharp)]
pub fn system_security_update_good_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SystemSecurityUpdateGoodSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M5,1v22h14V1H5z M17,18H7V6h10V18z M16,10.05l-1.41-1.41l-3.54,3.54l-1.41-1.41l-1.41,1.41L11.05,15L16,10.05z"/>
        </SvgIcon>
    }
}
