// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebStoriesSharp)]
pub fn web_stories_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebStoriesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,4c1.1,0,2,0,2,0v16c0,0-0.9,0-2,0V4z M2,2v20h13V2H2z M21,18c0.83,0,1.5,0,1.5,0V6c0,0-0.67,0-1.5,0V18z"/>
        </SvgIcon>
    }
}
