// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UTurnRightRounded)]
pub fn u_turn_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UTurnRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.29,12.29c-0.39-0.39-1.02-0.39-1.41,0L18,13.17V9c0-3.31-2.69-6-6-6S6,5.69,6,9v11c0,0.55,0.45,1,1,1s1-0.45,1-1V9 c0-2.21,1.79-4,4-4s4,1.79,4,4v4.17l-0.88-0.88c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l2.59,2.59 c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59C20.68,13.32,20.68,12.68,20.29,12.29z"/>
        </SvgIcon>
    }
}
