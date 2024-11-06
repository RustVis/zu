// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RamenDining)]
pub fn ramen_dining(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RamenDining"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,6H8V4.65l1-0.12V6z M9,12H8V7h1V12z M6,7h1v5H6V7z M6,4.88l1-0.12V6H6V4.88z M22,3V2L5,4v8H2c0,3.69,2.47,6.86,6,8.25 V22h8v-1.75c3.53-1.39,6-4.56,6-8.25H10V7h12V6H10V4.41L22,3z"/>
        </SvgIcon>
    }
}
