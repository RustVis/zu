// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PolylineRounded)]
pub fn polyline_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PolylineRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.04,6.85L7.3,10H4.5C3.67,10,3,10.67,3,11.5v3C3,15.33,3.67,16,4.5,16h3c0.14,0,0.27-0.02,0.39-0.05L15,19.5v1 c0,0.83,0.67,1.5,1.5,1.5h3c0.83,0,1.5-0.67,1.5-1.5v-3c0-0.83-0.67-1.5-1.5-1.5h-3c-0.75,0-1.37,0.55-1.48,1.27L9,14.26V11.5 c0-0.12-0.01-0.24-0.04-0.36L11.7,8h2.8C15.33,8,16,7.33,16,6.5v-3C16,2.67,15.33,2,14.5,2h-3C10.67,2,10,2.67,10,3.5v3 C10,6.62,10.01,6.74,10.04,6.85z"/>
        </SvgIcon>
    }
}
