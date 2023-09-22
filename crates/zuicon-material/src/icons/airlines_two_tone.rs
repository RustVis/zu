// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlinesTwoTone)]
pub fn airlines_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlinesTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.05,6L5.8,18h11.54l2.25-12H14.05z M14.5,14c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5 s2.5,1.12,2.5,2.5C17,12.88,15.88,14,14.5,14z" opacity=".3"/><path d="M17.34,18H5.8l8.25-12h5.54L17.34,18z M13,4L2,20h17l3-16H13z M14.5,9c-1.38,0-2.5,1.12-2.5,2.5s1.12,2.5,2.5,2.5 s2.5-1.12,2.5-2.5S15.88,9,14.5,9z"/>
        </SvgIcon>
    }
}
