// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NearbyOffSharp)]
pub fn nearby_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NearbyOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22.82,12.01L18.83,16l-1.81-1.81L19.2,12L12,4.8L9.81,6.99L8,5.17l3.99-3.99L22.82,12.01z M21.19,21.19l-1.41,1.41 L16,18.83l-3.99,3.99L1.18,11.99L5.17,8L1.39,4.22L2.8,2.81L21.19,21.19z M14.19,17.02l-1.39-1.39l-0.8,0.8L7.58,12l0.8-0.8 l-1.4-1.39L4.8,12l7.2,7.2L14.19,17.02z M16.42,12L12,7.58l-0.8,0.8l4.42,4.42L16.42,12z"/>
        </SvgIcon>
    }
}
