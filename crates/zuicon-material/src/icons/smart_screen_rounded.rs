// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartScreenRounded)]
pub fn smart_screen_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartScreenRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,5H3C1.9,5,1,5.9,1,7v10c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2V7C23,5.9,22.1,5,21,5z M18,7v10H6V7H18z M14,12 c0-0.41-0.34-0.75-0.75-0.75S12.5,11.59,12.5,12s0.34,0.75,0.75,0.75S14,12.41,14,12z M9,12c0-0.41-0.34-0.75-0.75-0.75 S7.5,11.59,7.5,12s0.34,0.75,0.75,0.75S9,12.41,9,12z M16.5,12c0-0.41-0.34-0.75-0.75-0.75S15,11.59,15,12s0.34,0.75,0.75,0.75 S16.5,12.41,16.5,12z M11.5,12c0-0.41-0.34-0.75-0.75-0.75S10,11.59,10,12s0.34,0.75,0.75,0.75S11.5,12.41,11.5,12z"/>
        </SvgIcon>
    }
}
