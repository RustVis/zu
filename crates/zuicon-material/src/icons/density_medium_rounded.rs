// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DensityMediumRounded)]
pub fn density_medium_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DensityMediumRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,5h16c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H4C3.45,3,3,3.45,3,4C3,4.55,3.45,5,4,5z"/><path d="M20,19H4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1C21,19.45,20.55,19,20,19z"/><path d="M20,11H4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1C21,11.45,20.55,11,20,11z"/>
        </SvgIcon>
    }
}
