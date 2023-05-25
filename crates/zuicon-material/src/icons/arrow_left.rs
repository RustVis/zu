// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowLeft)]
pub fn arrow_left(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowLeft"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14 7l-5 5 5 5V7z"/><path d="M24 0v24H0V0h24z" fill="none"/>
        </SvgIcon>
    }
}
