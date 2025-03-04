// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PostAddSharp)]
pub fn post_add_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PostAddSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,2h-2v3h-3c0.01,0.01,0,2,0,2h3v2.99c0.01,0.01,2,0,2,0V7h3V5h-3V2z"/>
        </SvgIcon>
    }
}
