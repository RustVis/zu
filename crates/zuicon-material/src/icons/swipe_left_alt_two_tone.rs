// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeLeftAltTwoTone)]
pub fn swipe_left_alt_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeLeftAltTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.1,13c0.46,2.28,2.48,4,4.9,4c2.76,0,5-2.24,5-5s-2.24-5-5-5c-2.42,0-4.44,1.72-4.9,4H5.83l1.59-1.59L6,8l-4,4l4,4 l1.41-1.41L5.83,13H10.1z M15,15c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S16.66,15,15,15z"/>
        </SvgIcon>
    }
}
