// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UTurnLeftRounded)]
pub fn u_turn_left_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UTurnLeftRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.71,12.29c0.39-0.39,1.02-0.39,1.41,0L6,13.17V9c0-3.31,2.69-6,6-6s6,2.69,6,6v11c0,0.55-0.45,1-1,1s-1-0.45-1-1V9 c0-2.21-1.79-4-4-4S8,6.79,8,9v4.17l0.88-0.88c0.39-0.39,1.02-0.39,1.41,0c0.39,0.39,0.39,1.02,0,1.41l-2.59,2.59 c-0.39,0.39-1.02,0.39-1.41,0l-2.59-2.59C3.32,13.32,3.32,12.68,3.71,12.29z"/>
        </SvgIcon>
    }
}
