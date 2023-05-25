// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HeadphonesBattery)]
pub fn headphones_battery(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HeadphonesBattery"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,7h-1V6h-2v1h-1c-0.55,0-1,0.45-1,1v9c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1V8C22,7.45,21.55,7,21,7z"/><path d="M8,6c-3.31,0-6,2.69-6,6v4c0,1.1,0.9,2,2,2h2v-5H3.5v-1c0-2.48,2.02-4.5,4.5-4.5s4.5,2.02,4.5,4.5v1H10v5h2 c1.1,0,2-0.9,2-2v-4C14,8.69,11.31,6,8,6z"/>
        </SvgIcon>
    }
}
