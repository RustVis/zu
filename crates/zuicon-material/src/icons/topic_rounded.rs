// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TopicRounded)]
pub fn topic_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TopicRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M13,16H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1C14,15.55,13.55,16,13,16z M17,12H7 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h10c0.55,0,1,0.45,1,1C18,11.55,17.55,12,17,12z"/>
        </SvgIcon>
    }
}
