// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElevatorSharp)]
pub fn elevator_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ElevatorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M8.5,6c0.69,0,1.25,0.56,1.25,1.25c0,0.69-0.56,1.25-1.25,1.25S7.25,7.94,7.25,7.25 C7.25,6.56,7.81,6,8.5,6z M11,14h-1v4H7v-4H6V9.5h5V14z M15.5,17L13,13h5L15.5,17z M13,11l2.5-4l2.5,4H13z"/>
        </SvgIcon>
    }
}
