// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Man2Rounded)]
pub fn man_2_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Man2Rounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,7h-4C8.9,7,8,7.9,8,9v5c0,0.55,0.45,1,1,1h1.5v5.5c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5V15H15 c0.55,0,1-0.45,1-1V9C16,7.9,15.1,7,14,7z"/>
        </SvgIcon>
    }
}
