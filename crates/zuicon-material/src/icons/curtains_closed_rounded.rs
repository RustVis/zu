// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurtainsClosedRounded)]
pub fn curtains_closed_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurtainsClosedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M11,5h2v14h-2V5z"/>
        </SvgIcon>
    }
}
