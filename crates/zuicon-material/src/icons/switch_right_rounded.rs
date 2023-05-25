// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwitchRightRounded)]
pub fn switch_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwitchRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.5,15.38V8.62L18.88,12L15.5,15.38 M20.29,12.71c0.39-0.39,0.39-1.02,0-1.41l-4.59-4.59C15.08,6.08,14,6.52,14,7.41v9.17 c0,0.89,1.08,1.34,1.71,0.71L20.29,12.71z M10,16.59V7.41c0-0.89-1.08-1.34-1.71-0.71l-4.59,4.59c-0.39,0.39-0.39,1.02,0,1.41 l4.59,4.59C8.92,17.92,10,17.48,10,16.59z"/>
        </SvgIcon>
    }
}
