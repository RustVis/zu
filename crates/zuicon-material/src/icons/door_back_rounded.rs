// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DoorBackRounded)]
pub fn door_back_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DoorBackRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,19h-1V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v14H4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1 C21,19.45,20.55,19,20,19z M10,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C11,12.55,10.55,13,10,13z"/>
        </SvgIcon>
    }
}
