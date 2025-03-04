// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebStoriesTwoTone)]
pub fn web_stories_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebStoriesTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,4v16c1.1,0,2-0.9,2-2V6C19,4.9,18.1,4,17,4z"/><path d="M13,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h9c1.1,0,2-0.9,2-2V4C15,2.9,14.1,2,13,2z M13,20H4V4h9V20z"/><path d="M21,6v12c0.83,0,1.5-0.67,1.5-1.5v-9C22.5,6.67,21.83,6,21,6z"/>
        </SvgIcon>
    }
}
