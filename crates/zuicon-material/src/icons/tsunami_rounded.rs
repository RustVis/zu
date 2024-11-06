// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TsunamiRounded)]
pub fn tsunami_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TsunamiRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.16,17.98c-2.76,1.76-4.67,0.77-5.61,0.08c-0.34-0.24-0.78-0.23-1.12,0.01c-0.97,0.7-2.83,1.65-5.55-0.06 C5.55,17.8,5.13,17.78,4.81,18c-0.91,0.61-1.53,0.85-2,0.94C2.34,19.03,2,19.44,2,19.91v0c0,0.6,0.54,1.09,1.13,0.98 c0.77-0.14,1.51-0.42,2.2-0.83c2.04,1.21,4.63,1.21,6.67,0c2.06,1.22,4.61,1.22,6.67,0c0.69,0.41,1.44,0.69,2.21,0.83 c0.59,0.11,1.13-0.38,1.13-0.98v-0.01c0-0.47-0.33-0.88-0.8-0.97c-0.49-0.1-1.11-0.34-2.02-0.94 C18.88,17.79,18.47,17.78,18.16,17.98z"/><path d="M19.33,12H21c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1l-1.61,0c-1.86,0-3.4-1.5-3.39-3.36c0-0.37,0.06-0.7,0.16-1.05 C16.53,4.3,15.6,3.03,14.27,3C14.18,3,14.09,3,14,3C7.36,3,2.15,8.03,2.01,14.5l0,0.03c-0.04,1.13,1.07,1.98,2.14,1.6 c0.4-0.14,0.78-0.32,1.15-0.54c2.08,1.2,4.64,1.22,6.7-0.02c2.06,1.22,4.61,1.22,6.67,0c0.68,0.41,1.42,0.68,2.18,0.82 c0.6,0.11,1.16-0.36,1.16-0.98v-0.01c0-0.46-0.32-0.88-0.78-0.97c-0.49-0.09-1.12-0.33-2.03-0.94c-0.31-0.21-0.73-0.22-1.05-0.01 c-2.73,1.74-4.63,0.77-5.58,0.09c-0.35-0.25-0.81-0.26-1.16-0.01c-0.15,0.11-0.09,0.06-0.32,0.2C10.39,12.82,10,11.7,10,10.5 c0-2.58,1.77-4.74,4.21-5.33C14.08,5.68,14,6.19,14,6.67C14,9.61,16.39,12,19.33,12z"/>
        </SvgIcon>
    }
}
