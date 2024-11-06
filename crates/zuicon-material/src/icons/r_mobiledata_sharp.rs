// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RMobiledataSharp)]
pub fn r_mobiledata_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RMobiledataSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M7.8,7.2L9,10H7L5.87,7.33H4V10H2V2h7v5.2H7.8z M7,4H4v1.33h3V4z"/>
        </SvgIcon>
    }
}
