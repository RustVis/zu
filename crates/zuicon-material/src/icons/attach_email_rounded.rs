// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AttachEmailRounded)]
pub fn attach_email_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AttachEmailRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,10V4c0-1.1-0.9-2-2-2H3C1.9,2,1.01,2.9,1.01,4L1,16c0,1.1,0.9,2,2,2h11v-5c0-1.66,1.34-3,3-3H21z M11.53,10.67 c-0.32,0.2-0.74,0.2-1.06,0L3.4,6.25C3.15,6.09,3,5.82,3,5.53c0-0.67,0.73-1.07,1.3-0.72L11,9l6.7-4.19 C18.27,4.46,19,4.86,19,5.53c0,0.29-0.15,0.56-0.4,0.72L11.53,10.67z"/><path d="M22,14c-0.55,0-1,0.45-1,1v3c0,1.1-0.9,2-2,2s-2-0.9-2-2v-4.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V17 c0,0.55,0.45,1,1,1s1-0.45,1-1v-3.5c0-1.38-1.12-2.5-2.5-2.5S15,12.12,15,13.5V18c0,2.21,1.79,4,4,4s4-1.79,4-4v-3 C23,14.45,22.55,14,22,14z"/>
        </SvgIcon>
    }
}
