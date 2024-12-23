// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EscalatorWarningRounded)]
pub fn escalator_warning_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EscalatorWarningRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.5,2c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S5.4,2,6.5,2z M15.5,9.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5 S17.83,8,17,8S15.5,8.67,15.5,9.5z M18.5,12h-2.84c-0.58,0.01-1.14,0.32-1.45,0.86l-0.92,1.32L9.72,8C9.35,7.37,8.69,7.01,8.01,7H5 C3.9,7,3,7.9,3,9v5c0,0.55,0.45,1,1,1h0.5v6c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-9.39l2.24,3.89c0.18,0.31,0.51,0.5,0.87,0.5 h1.1c0.33,0,0.63-0.16,0.82-0.43L15,14.9V21c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-4h0c0.55,0,1-0.45,1-1v-2.5 C20,12.68,19.33,12,18.5,12z"/>
        </SvgIcon>
    }
}
