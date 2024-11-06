// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MoveUpTwoTone)]
pub fn move_up_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MoveUpTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,13c0-2.45,1.76-4.47,4.08-4.91L5.59,9.59L7,11l4-4.01L7,3L5.59,4.41l1.58,1.58l0,0.06C3.7,6.46,1,9.42,1,13 c0,3.87,3.13,7,7,7h3v-2H8C5.24,18,3,15.76,3,13z"/><path d="M13,13v7h9v-7H13z M20,18h-5v-3h5V18z"/>
        </SvgIcon>
    }
}
