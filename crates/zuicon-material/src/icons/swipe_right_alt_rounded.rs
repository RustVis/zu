// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeRightAltRounded)]
pub fn swipe_right_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeRightAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.9,11C13.44,8.72,11.42,7,9,7c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.42,0,4.44-1.72,4.9-4h4.27l-0.88,0.88 c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59 c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L18.17,11H13.9z"/>
        </SvgIcon>
    }
}
