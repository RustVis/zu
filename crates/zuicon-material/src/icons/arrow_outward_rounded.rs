// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowOutwardRounded)]
pub fn arrow_outward_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowOutwardRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,7L6,7c0,0.55,0.45,1,1,1h7.59l-8.88,8.88c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0L16,9.41V17 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V7c0-0.55-0.45-1-1-1H7C6.45,6,6,6.45,6,7z"/>
        </SvgIcon>
    }
}
