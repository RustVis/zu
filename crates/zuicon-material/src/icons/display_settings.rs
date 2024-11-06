// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DisplaySettings)]
pub fn display_settings(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DisplaySettings"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,3H4C2.89,3,2,3.89,2,5v12c0,1.1,0.89,2,2,2h4v2h8v-2h4c1.1,0,2-0.9,2-2V5C22,3.89,21.1,3,20,3z M20,17H4V5h16V17z"/>
        </SvgIcon>
    }
}
