// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UTurnRightSharp)]
pub fn u_turn_right_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UTurnRightSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,9v12h2V9c0-2.21,1.79-4,4-4s4,1.79,4,4v4.17l-1.59-1.59L13,13l4,4l4-4l-1.41-1.41L18,13.17V9c0-3.31-2.69-6-6-6 S6,5.69,6,9z"/>
        </SvgIcon>
    }
}
