// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DesktopMac)]
pub fn desktop_mac(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DesktopMac"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20 3H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h6l-2 2v1h8v-1l-2-2h6c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2"/>
        </SvgIcon>
    }
}
