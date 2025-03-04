// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MediationRounded)]
pub fn mediation_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MediationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,13h-5.06c-0.34,3.1-2.26,5.72-4.94,7.05c-0.03,1.81-1.66,3.23-3.55,2.9c-1.2-0.21-2.19-1.2-2.4-2.4 C1.71,18.65,3.16,17,5,17c0.95,0,1.78,0.45,2.33,1.14c1.9-1.03,3.26-2.91,3.58-5.14h-3.1c-0.48,1.34-1.86,2.24-3.42,1.94 c-1.18-0.23-2.13-1.2-2.35-2.38C1.7,10.66,3.16,9,5,9c1.3,0,2.4,0.84,2.82,2h3.1C10.6,8.77,9.23,6.9,7.33,5.86 c-0.64,0.8-1.67,1.28-2.81,1.1C3.29,6.77,2.26,5.77,2.05,4.54C1.72,2.65,3.17,1,5,1c1.64,0,2.96,1.31,2.99,2.95 c2.68,1.33,4.6,3.95,4.94,7.05H18V9.21c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79 C18.54,15.46,18,15.24,18,14.79V13z"/>
        </SvgIcon>
    }
}
