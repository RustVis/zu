// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IceSkatingSharp)]
pub fn ice_skating_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IceSkatingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,17c0,1.66-1.34,3-3,3h-2v-2h3l-0.01-6l-5.71-1.43C12.4,10.35,11.7,9.76,11.32,9H8V8h3.02L11,7H8V6h3V3H3v15h3v2H2v2h16 c2.76,0,5-2.24,5-5H21z M14,20H8v-2h6V20z"/>
        </SvgIcon>
    }
}
