// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HandshakeSharp)]
pub fn handshake_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HandshakeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.59,5.95l-7.05,7.04L0.7,10.3l8.55-8.55l7.95,7.95l-1.42,1.42L10.59,5.95z M23.24,10.24l-8.49-8.49l-2.06,2.06l5.9,5.88 l-2.83,2.83l-5.17-5.17l-6.27,6.27l1.42,1.41l5.32-5.32l0.71,0.71l-5.32,5.32l1.42,1.41l5.32-5.32l0.71,0.71l-5.32,5.32l1.41,1.41 l5.32-5.32l0.71,0.71L10.68,20l1.41,1.41L23.24,10.24z"/>
        </SvgIcon>
    }
}
