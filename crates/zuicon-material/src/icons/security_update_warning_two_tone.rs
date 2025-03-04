// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SecurityUpdateWarningTwoTone)]
pub fn security_update_warning_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SecurityUpdateWarningTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M11,7h2v6h-2V7z M11,15h2v2h-2V15z" opacity=".3"/><path d="M17,1.01L7,1C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1.01,17,1.01z M17,21H7v-1h10V21z M17,18H7V6h10V18z M17,4H7V3h10V4z"/><path d="M7,21h10v-1H7V21z M7,3v1h10V3H7z" opacity=".3"/>
        </SvgIcon>
    }
}
