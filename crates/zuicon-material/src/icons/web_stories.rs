// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebStories)]
pub fn web_stories(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebStories"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,4c1.1,0,2,0.9,2,2v12c0,1.1-0.9,2-2,2V4z M2,20c0,1.1,0.9,2,2,2h9c1.1,0,2-0.9,2-2V4c0-1.1-0.9-2-2-2H4 C2.9,2,2,2.9,2,4V20z M21,18c0.83,0,1.5-0.67,1.5-1.5v-9C22.5,6.67,21.83,6,21,6V18z"/>
        </SvgIcon>
    }
}
