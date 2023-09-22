// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AdUnitsSharp)]
pub fn ad_units_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AdUnitsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,1H5v22h14V1z M17,19H7V5h10V19z"/>
        </SvgIcon>
    }
}
