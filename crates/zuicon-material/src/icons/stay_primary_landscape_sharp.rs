// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StayPrimaryLandscapeSharp)]
pub fn stay_primary_landscape_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StayPrimaryLandscapeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M1 19h22V5H1v14zM19 7v10H5V7h14z"/>
        </SvgIcon>
    }
}
