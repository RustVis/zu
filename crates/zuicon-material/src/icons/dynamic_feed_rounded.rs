// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DynamicFeedRounded)]
pub fn dynamic_feed_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DynamicFeedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,8L7,8C6.45,8,6,8.45,6,9v6c0,1.1,0.9,2,2,2h8c0.55,0,1-0.45,1-1l0,0c0-0.55-0.45-1-1-1H8V9C8,8.45,7.55,8,7,8z"/><path d="M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z"/><path d="M3,12L3,12c-0.55,0-1,0.45-1,1v6c0,1.1,0.9,2,2,2h8c0.55,0,1-0.45,1-1l0,0c0-0.55-0.45-1-1-1H4v-6C4,12.45,3.55,12,3,12z"/>
        </SvgIcon>
    }
}
