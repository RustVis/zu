// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NineKPlusRounded)]
pub fn nine_k_plus_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NineKPlusRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7.5,10h1v1.5h-1V10z M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M10,14 c0,0.55-0.45,1-1,1H6.75C6.34,15,6,14.66,6,14.25v0c0-0.41,0.34-0.75,0.75-0.75H8.5v-1H7c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1 h2c0.55,0,1,0.45,1,1V14z M14.59,15L14.59,15c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.55c0,0.39-0.31,0.7-0.7,0.7H11.7 c-0.39,0-0.7-0.31-0.7-0.7V9.7C11,9.31,11.31,9,11.7,9h0.09c0.39,0,0.7,0.31,0.7,0.7v1.55l1.54-1.98C14.17,9.1,14.38,9,14.59,9h0 c0.58,0,0.91,0.66,0.56,1.12L13.75,12l1.41,1.88C15.5,14.34,15.17,15,14.59,15z M18.5,12.5h-1v1c0,0.28-0.22,0.5-0.5,0.5l0,0 c-0.28,0-0.5-0.22-0.5-0.5v-1h-1c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5h1v-1c0-0.28,0.22-0.5,0.5-0.5l0,0 c0.28,0,0.5,0.22,0.5,0.5v1h1c0.28,0,0.5,0.22,0.5,0.5v0C19,12.28,18.78,12.5,18.5,12.5z"/>
        </SvgIcon>
    }
}