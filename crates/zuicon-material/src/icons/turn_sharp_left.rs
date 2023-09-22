// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnSharpLeft)]
pub fn turn_sharp_left(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnSharpLeft"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,6.83L4.41,8.41L3,7l4-4l4,4L9.59,8.41L8,6.83V13h8c1.1,0,2,0.9,2,2v6h-2v-6H8c-1.1,0-2-0.9-2-2V6.83z"/>
        </SvgIcon>
    }
}
