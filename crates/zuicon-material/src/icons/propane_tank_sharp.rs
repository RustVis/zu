// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PropaneTankSharp)]
pub fn propane_tank_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PropaneTankSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,15v3c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-3H4z"/><path d="M20,13v-3c0-1.86-1.28-3.41-3-3.86V2H7v4.14c-1.72,0.45-3,2-3,3.86v3H20z M9,4h6v2h-2V5h-2v1H9V4z"/>
        </SvgIcon>
    }
}
