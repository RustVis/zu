// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScienceRounded)]
pub fn science_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScienceRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.54,17.73L15,11V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H8C7.45,3,7,3.45,7,4s0.45,1,1,1h1v6l-5.54,6.73 C3.14,18.12,3,18.56,3,19c0.01,1.03,0.82,2,2,2H19c1.19,0,2-0.97,2-2C21,18.56,20.86,18.12,20.54,17.73z"/>
        </SvgIcon>
    }
}
