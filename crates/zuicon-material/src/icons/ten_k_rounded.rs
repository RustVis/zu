// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TenKRounded)]
pub fn ten_k_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TenKRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,10.5h1.5v3H10V10.5z M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M6.75,15 L6.75,15C6.34,15,6,14.66,6,14.25V10.5H5.25c-0.41,0-0.75-0.34-0.75-0.75v0C4.5,9.34,4.84,9,5.25,9H6.5c0.55,0,1,0.45,1,1v4.25 C7.5,14.66,7.16,15,6.75,15z M13,14c0,0.55-0.45,1-1,1H9.5c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1H12c0.55,0,1,0.45,1,1V14z M17.59,15L17.59,15c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.5c0,0.41-0.34,0.75-0.75,0.75h0C14.34,15,14,14.66,14,14.25v-4.5 C14,9.34,14.34,9,14.75,9h0c0.41,0,0.75,0.34,0.75,0.75v1.5l1.54-1.98C17.17,9.1,17.38,9,17.59,9l0,0c0.58,0,0.91,0.66,0.56,1.12 L16.75,12l1.41,1.88C18.5,14.34,18.17,15,17.59,15z"/>
        </SvgIcon>
    }
}
