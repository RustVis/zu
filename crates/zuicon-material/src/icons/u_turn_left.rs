// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UTurnLeft)]
pub fn u_turn_left(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UTurnLeft"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,9v12h-2V9c0-2.21-1.79-4-4-4S8,6.79,8,9v4.17l1.59-1.59L11,13l-4,4l-4-4l1.41-1.41L6,13.17V9c0-3.31,2.69-6,6-6 S18,5.69,18,9z"/>
        </SvgIcon>
    }
}
