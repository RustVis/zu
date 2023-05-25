// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnSharpRightRounded)]
pub fn turn_sharp_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnSharpRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,6.83l-0.88,0.88c-0.39,0.39-1.02,0.39-1.41,0c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0 l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41c-0.39,0.39-1.02,0.39-1.41,0L18,6.83V13c0,1.1-0.9,2-2,2H8v5c0,0.55-0.45,1-1,1h0 c-0.55,0-1-0.45-1-1v-5c0-1.1,0.9-2,2-2h8V6.83L16,6.83z"/>
        </SvgIcon>
    }
}
