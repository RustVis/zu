// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwitchLeftRounded)]
pub fn switch_left_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwitchLeftRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.5,8.62v6.76L5.12,12L8.5,8.62 M3.71,11.29c-0.39,0.39-0.39,1.02,0,1.41l4.59,4.59C8.92,17.92,10,17.48,10,16.59V7.41 c0-0.89-1.08-1.34-1.71-0.71L3.71,11.29z M14,7.41v9.17c0,0.89,1.08,1.34,1.71,0.71l4.59-4.59c0.39-0.39,0.39-1.02,0-1.41 l-4.59-4.59C15.08,6.08,14,6.52,14,7.41z"/>
        </SvgIcon>
    }
}
