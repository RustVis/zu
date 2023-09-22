// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EightKRounded)]
pub fn eight_k_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EightKRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,12.5h1.5V14H8V12.5z M8,10h1.5v1.5H8V10z M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5 C21,3.9,20.1,3,19,3z M11,14c0,0.55-0.45,1-1,1H7.5c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1H10c0.55,0,1,0.45,1,1V14z M16.59,15 L16.59,15c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.55c0,0.39-0.31,0.7-0.7,0.7H13.7c-0.39,0-0.7-0.31-0.7-0.7V9.7 C13,9.31,13.31,9,13.7,9h0.09c0.39,0,0.7,0.31,0.7,0.7v1.55l1.54-1.98C16.17,9.1,16.38,9,16.59,9l0,0c0.58,0,0.91,0.66,0.56,1.12 L15.75,12l1.41,1.88C17.5,14.34,17.17,15,16.59,15z"/>
        </SvgIcon>
    }
}
