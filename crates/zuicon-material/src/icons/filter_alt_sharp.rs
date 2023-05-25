// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FilterAltSharp)]
pub fn filter_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FilterAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24 M24,24H0" fill="none"/><path d="M3,4c2.01,2.59,7,9,7,9v7h4v-7c0,0,4.98-6.41,7-9H3z"/><path d="M0,0h24v24H0V0z" fill="none"/>
        </SvgIcon>
    }
}
