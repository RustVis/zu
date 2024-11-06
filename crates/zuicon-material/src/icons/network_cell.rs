// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkCell)]
pub fn network_cell(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkCell"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,22h20V2L2,22z M20,20h-3V9.83l3-3V20z"/>
        </SvgIcon>
    }
}
