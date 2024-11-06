// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThreeGMobiledataSharp)]
pub fn three_g_mobiledata_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThreeGMobiledataSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M3,7v2h5v2H4v2h4v2H3v2h7V7H3z M21,11v6h-9V7h9v2h-7v6h5v-2h-2.5v-2H21z"/>
        </SvgIcon>
    }
}
