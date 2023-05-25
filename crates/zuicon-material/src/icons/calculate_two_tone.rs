// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CalculateTwoTone)]
pub fn calculate_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CalculateTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14V5H5V19z M13.03,7.06L14.09,6l1.41,1.41L16.91,6l1.06,1.06l-1.41,1.41l1.41,1.41l-1.06,1.06 L15.5,9.54l-1.41,1.41l-1.06-1.06l1.41-1.41L13.03,7.06z M13,13.25h5v1.5h-5V13.25z M13,15.75h5v1.5h-5V15.75z M6.25,7.72h5v1.5 h-5V7.72z M6,14.5h2v-2h1.5v2h2V16h-2v2H8v-2H6V14.5z" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/>
        </SvgIcon>
    }
}
