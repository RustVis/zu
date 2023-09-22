// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HourglassTop)]
pub fn hourglass_top(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HourglassTop"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,2l0.01,6L10,12l-3.99,4.01L6,22h12v-6l-4-4l4-3.99V2H6z M16,16.5V20H8v-3.5l4-4L16,16.5z"/>
        </SvgIcon>
    }
}
