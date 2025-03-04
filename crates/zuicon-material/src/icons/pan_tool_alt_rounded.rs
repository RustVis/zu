// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanToolAltRounded)]
pub fn pan_tool_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanToolAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5.2,15.43c0-0.65,0.6-1.13,1.24-0.99L10,15.24V4.5C10,3.67,10.67,3,11.5,3S13,3.67,13,4.5v6h0.91 c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04c0.77,0.38,1.21,1.22,1.09,2.07l-0.63,4.46C19.21,20.27,18.36,21,17.37,21h-6.16 c-0.53,0-1.29-0.21-1.66-0.59l-4.07-4.29C5.3,15.94,5.2,15.69,5.2,15.43z"/>
        </SvgIcon>
    }
}
