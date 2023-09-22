// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DesktopMacSharp)]
pub fn desktop_mac_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DesktopMacSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M23 2H1v16h9l-2 3v1h8v-1l-2-3h9V2zm-2 12H3V4h18v10z"/>
        </SvgIcon>
    }
}
