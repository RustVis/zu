// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FaxRounded)]
pub fn fax_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FaxRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9h-1V6c0-1.1-0.9-2-2-2h-6C8.9,4,8,4.9,8,6v14h12c1.1,0,2-0.9,2-2v-6C22,10.34,20.66,9,19,9z M10,6h6v3h-6V6z M14,17 h-4v-5h4V17z M16,17c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C17,16.55,16.55,17,16,17z M16,14c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C17,13.55,16.55,14,16,14z M19,17c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C20,16.55,19.55,17,19,17z M19,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C20,13.55,19.55,14,19,14z"/><path d="M4.5,8C3.12,8,2,9.12,2,10.5v8C2,19.88,3.12,21,4.5,21S7,19.88,7,18.5v-8C7,9.12,5.88,8,4.5,8z"/>
        </SvgIcon>
    }
}
