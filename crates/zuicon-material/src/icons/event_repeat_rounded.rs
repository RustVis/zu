// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EventRepeatRounded)]
pub fn event_repeat_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EventRepeatRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,12V6c0-1.1-0.9-2-2-2h-1V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v1H8V3c0-0.55-0.45-1-1-1S6,2.45,6,3v1H5C3.9,4,3,4.9,3,6v14 c0,1.1,0.9,2,2,2h7v-2H5V10h14v2H21z M15.13,20c-0.55,0-0.91,0.56-0.68,1.06C15.23,22.79,16.97,24,19,24c2.76,0,5-2.24,5-5 s-2.24-5-5-5c-1.36,0-2.6,0.55-3.5,1.43l0-0.68c0-0.41-0.34-0.75-0.75-0.75h0C14.34,14,14,14.34,14,14.75V17c0,0.55,0.45,1,1,1 h2.25c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75l-0.7,0c0.63-0.62,1.5-1,2.45-1c1.93,0,3.5,1.57,3.5,3.5 s-1.57,3.5-3.5,3.5c-1.42,0-2.64-0.85-3.19-2.06C15.69,20.17,15.42,20,15.13,20z"/>
        </SvgIcon>
    }
}
