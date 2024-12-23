// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThirtyFpsRounded)]
pub fn thirty_fps_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThirtyFpsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M2,6.5L2,6.5C2,7.33,2.67,8,3.5,8H8v2.5H4.5C3.67,10.5,3,11.17,3,12v0c0,0.83,0.67,1.5,1.5,1.5H8V16H3.5 C2.67,16,2,16.67,2,17.5v0C2,18.33,2.67,19,3.5,19H8c1.66,0,3-1.34,3-3v-1.9c0-1.16-0.94-2.1-2.1-2.1c1.16,0,2.1-0.94,2.1-2.1V8 c0-1.66-1.34-3-3-3H3.5C2.67,5,2,5.67,2,6.5z M19,8v8h-4V8H19 M19,5h-4c-1.66,0-3,1.34-3,3v8c0,1.66,1.34,3,3,3h4 c1.66,0,3-1.34,3-3V8C22,6.34,20.66,5,19,5z"/>
        </SvgIcon>
    }
}
