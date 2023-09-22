// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DockSharp)]
pub fn dock_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DockSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 23h8v-2H8v2zM18 1.01L6 1v18h12V1.01zM16 15H8V5h8v10z"/>
        </SvgIcon>
    }
}
