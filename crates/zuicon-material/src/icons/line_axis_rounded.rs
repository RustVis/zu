// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LineAxisRounded)]
pub fn line_axis_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LineAxisRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.34,6.77L21.34,6.77c-0.4-0.4-1.07-0.39-1.45,0.04l-3.33,3.74l-5.65-5.24C10.12,4.58,8.9,4.6,8.14,5.36L2.7,10.81 c-0.39,0.39-0.39,1.02,0,1.41l0.09,0.09c0.39,0.39,1.02,0.39,1.41,0l5.44-5.45l5.59,5.19l-1.73,1.95l-2.58-2.58 c-0.78-0.78-2.05-0.78-2.83,0L2.7,16.8c-0.39,0.39-0.39,1.02,0,1.41L2.8,18.3c0.39,0.39,1.02,0.39,1.41,0l5.3-5.3l2.5,2.5 c0.81,0.81,2.14,0.77,2.91-0.09l1.78-2.01l3.19,2.96c0.39,0.36,1,0.35,1.38-0.03l0.01-0.01c0.4-0.4,0.39-1.05-0.03-1.43l-3.22-2.99 l3.35-3.77C21.73,7.74,21.71,7.14,21.34,6.77z"/>
        </SvgIcon>
    }
}
