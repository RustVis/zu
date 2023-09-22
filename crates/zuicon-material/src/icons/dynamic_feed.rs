// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DynamicFeed)]
pub fn dynamic_feed(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DynamicFeed"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,8H6v7c0,1.1,0.9,2,2,2h9v-2H8V8z"/><path d="M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z"/><path d="M4,12H2v7c0,1.1,0.9,2,2,2h9v-2H4V12z"/><path d="M8,8H6v7c0,1.1,0.9,2,2,2h9v-2H8V8z"/><path d="M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z"/><path d="M4,12H2v7c0,1.1,0.9,2,2,2h9v-2H4V12z"/>
        </SvgIcon>
    }
}
