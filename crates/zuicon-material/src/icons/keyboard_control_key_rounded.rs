// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardControlKeyRounded)]
pub fn keyboard_control_key_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KeyboardControlKeyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5.71,12.71L5.71,12.71c0.39,0.39,1.02,0.39,1.41,0L12,7.83l4.88,4.88c0.39,0.39,1.02,0.39,1.41,0v0 c0.39-0.39,0.39-1.02,0-1.41l-5.59-5.59c-0.39-0.39-1.02-0.39-1.41,0l-5.59,5.59C5.32,11.68,5.32,12.32,5.71,12.71z"/>
        </SvgIcon>
    }
}
