// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MopedRounded)]
pub fn moped_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MopedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,7c0-1.1-0.9-2-2-2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2.65L13.52,14H10v-4c0-0.55-0.45-1-1-1H6 c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35V7z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z"/><path d="M9,6H6C5.45,6,5,6.45,5,7v0c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v0C10,6.45,9.55,6,9,6z"/><path d="M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,17,19,17z"/>
        </SvgIcon>
    }
}
