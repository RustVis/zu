// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LivingSharp)]
pub fn living_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LivingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,2H2v20h20V2z M19,9.99V18H5v-8l1.25-0.01V6h11.5v3.99H19z"/>
        </SvgIcon>
    }
}
