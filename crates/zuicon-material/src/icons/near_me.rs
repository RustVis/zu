// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NearMe)]
pub fn near_me(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NearMe"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 3L3 10.53v.98l6.84 2.65L12.48 21h.98L21 3z"/>
        </SvgIcon>
    }
}
