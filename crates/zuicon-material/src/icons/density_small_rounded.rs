// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DensitySmallRounded)]
pub fn density_small_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DensitySmallRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3L3,3c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4C3.45,2,3,2.45,3,3z"/><path d="M4,22h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4c-0.55,0-1,0.45-1,1v0C3,21.55,3.45,22,4,22z"/><path d="M4,16h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4c-0.55,0-1,0.45-1,1v0C3,15.55,3.45,16,4,16z"/><path d="M4,10h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4C3.45,8,3,8.45,3,9v0C3,9.55,3.45,10,4,10z"/>
        </SvgIcon>
    }
}
