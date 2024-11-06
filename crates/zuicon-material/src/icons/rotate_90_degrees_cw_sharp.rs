// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Rotate90DegreesCwSharp)]
pub fn rotate_90_degrees_cw_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Rotate90DegreesCwSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4.64,19.37c3.03,3.03,7.67,3.44,11.15,1.25l-1.46-1.46c-2.66,1.43-6.04,1.03-8.28-1.21c-2.73-2.73-2.73-7.17,0-9.9 C7.42,6.69,9.21,6.03,11,6.03V9l4-4l-4-4v3.01c-2.3,0-4.61,0.87-6.36,2.63C1.12,10.15,1.12,15.85,4.64,19.37z"/><path d="M17,7l-6,6l6,6l6-6L17,7z"/>
        </SvgIcon>
    }
}
