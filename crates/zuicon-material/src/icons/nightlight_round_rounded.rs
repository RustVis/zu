// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NightlightRoundRounded)]
pub fn nightlight_round_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NightlightRoundRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.5,22c0.07,0,0.14,0,0.21,0c0.84-0.02,1.12-1.11,0.41-1.56c-2.78-1.77-4.63-4.89-4.63-8.43c0-3.55,1.85-6.66,4.63-8.44 c0.7-0.45,0.44-1.54-0.39-1.56c-0.04,0-0.09,0-0.13,0c-4.9-0.05-9.21,3.53-9.98,8.37C4.64,16.61,9.45,22,15.5,22L15.5,22z"/>
        </SvgIcon>
    }
}
