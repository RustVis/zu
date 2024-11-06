// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardAltRounded)]
pub fn keyboard_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KeyboardAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,4H3C1.9,4,1,4.9,1,6v13c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2V6C23,4.9,22.1,4,21,4z M7,12v2H5v-2H7z M5,10V8h2v2H5z M11,12v2H9v-2H11z M9,10V8h2v2H9z M16,16.5L16,16.5c0,0.28-0.22,0.5-0.5,0.5h-7C8.22,17,8,16.78,8,16.5l0,0 C8,16.22,8.22,16,8.5,16h7C15.78,16,16,16.22,16,16.5z M15,12v2h-2v-2H15z M13,10V8h2v2H13z M17,14v-2h2v2H17z M19,10h-2V8h2V10z"/>
        </SvgIcon>
    }
}
