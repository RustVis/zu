// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HourglassBottomTwoTone)]
pub fn hourglass_bottom_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HourglassBottomTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,22h12v-6l-4-4l3.99-4.01L18,2H6l0.01,5.99L10,12l-4,3.99V22z M8,7.5V4h8v3.5l-4,4L8,7.5z M8,16.5l4-4l4,4V20H8V16.5z"/>
        </SvgIcon>
    }
}
