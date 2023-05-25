// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TransitEnterexit)]
pub fn transit_enterexit(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TransitEnterexit"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M16 18H6V8h3v4.77L15.98 6 18 8.03 11.15 15H16v3z"/>
        </SvgIcon>
    }
}
