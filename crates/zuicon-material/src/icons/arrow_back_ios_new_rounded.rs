// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowBackIosNewRounded)]
pub fn arrow_back_ios_new_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowBackIosNewRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.88,2.88L16.88,2.88c-0.49-0.49-1.28-0.49-1.77,0l-8.41,8.41c-0.39,0.39-0.39,1.02,0,1.41l8.41,8.41 c0.49,0.49,1.28,0.49,1.77,0l0,0c0.49-0.49,0.49-1.28,0-1.77L9.54,12l7.35-7.35C17.37,4.16,17.37,3.37,16.88,2.88z"/>
        </SvgIcon>
    }
}
