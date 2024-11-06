// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RollerShadesClosedSharp)]
pub fn roller_shades_closed_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RollerShadesClosedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V3H4v16H2v2h8.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M6,19v-2h5v2H6z M13,19v-2h5v2H13z"/>
        </SvgIcon>
    }
}
