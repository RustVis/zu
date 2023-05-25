// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlightClassRounded)]
pub fn flight_class_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlightClassRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,4h-2c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2V6C18,4.9,17.1,4,16,4z M6,4c0.55,0,1,0.45,1,1v3l2.5,8H17 c0.55,0,1,0.45,1,1s-0.45,1-1,1H9.49c-0.88,0-1.66-0.58-1.92-1.43L5.08,8.28C5.03,8.09,5,7.9,5,7.71V5C5,4.45,5.45,4,6,4z M18,20 c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1s0.45-1,1-1h8C17.55,19,18,19.45,18,20z"/>
        </SvgIcon>
    }
}
