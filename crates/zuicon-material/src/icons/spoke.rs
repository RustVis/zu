// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Spoke)]
pub fn spoke(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Spoke"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,7c0,2.21-1.79,4-4,4S8,9.21,8,7s1.79-4,4-4S16,4.79,16,7z M7,13c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S9.21,13,7,13 z M17,13c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S19.21,13,17,13z"/>
        </SvgIcon>
    }
}
