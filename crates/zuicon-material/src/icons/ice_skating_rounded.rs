// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IceSkatingRounded)]
pub fn ice_skating_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IceSkatingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.87,17c-0.47,0-0.85,0.34-0.98,0.8C20.54,19.07,19.38,20,18,20h-2v-2h1c1.1,0,2-0.9,2-2l0-0.88 c0-2.1-1.55-3.53-3.03-3.88l-2.7-0.67C12.4,10.35,11.7,9.76,11.32,9H8.5C8.22,9,8,8.78,8,8.5S8.22,8,8.5,8h2.52L11,7H8.5 C8.22,7,8,6.78,8,6.5C8,6.22,8.22,6,8.5,6H11V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v11c0,1.1,0.9,2,2,2h1v2H3c-0.55,0-1,0.45-1,1 s0.45,1,1,1h15c2.33,0,4.29-1.6,4.84-3.75C23.01,17.62,22.52,17,21.87,17z M14,20H8v-2h6V20z"/>
        </SvgIcon>
    }
}
