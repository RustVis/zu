// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AdUnitsRounded)]
pub fn ad_units_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AdUnitsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M16,19H8c-0.55,0-1-0.45-1-1V6 c0-0.55,0.45-1,1-1h8c0.55,0,1,0.45,1,1v12C17,18.55,16.55,19,16,19z"/><path d="M15,6H9C8.45,6,8,6.45,8,7v0c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v0C16,6.45,15.55,6,15,6z"/>
        </SvgIcon>
    }
}
