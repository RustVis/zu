// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeLeftAlt)]
pub fn swipe_left_alt(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeLeftAlt"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.1,13c0.46,2.28,2.48,4,4.9,4c2.76,0,5-2.24,5-5s-2.24-5-5-5c-2.42,0-4.44,1.72-4.9,4H5.83l1.59-1.59L6,8l-4,4l4,4 l1.41-1.41L5.83,13H10.1z"/>
        </SvgIcon>
    }
}
