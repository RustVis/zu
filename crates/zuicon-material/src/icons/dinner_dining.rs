// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DinnerDining)]
pub fn dinner_dining(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DinnerDining"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,19h20l-2,2H4L2,19z M5,6h1v1H5V6z M5,4h1v1H5V4z M9,4v1H7V4H9z M9,7H7V6h2V7z M6,15.23c-0.36,0.11-0.69,0.28-1,0.47V8h1 V15.23z M4,16.52C3.62,16.96,3.32,17.45,3.16,18h16.82c0.01-0.16,0.03-0.33,0.03-0.5c0-3.04-2.46-5.5-5.5-5.5 c-2.29,0-4.25,1.4-5.08,3.4C8.84,15.15,8.19,15,7.5,15c-0.17,0-0.33,0.02-0.5,0.04V8h2c1.03,0.06,1.9-0.96,2-2h10V5H11 c-0.1-1.05-0.97-1.97-2-2H3v1h1v1H3v1h1v1H3v1h1V16.52z"/>
        </SvgIcon>
    }
}
