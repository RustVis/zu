// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SynagogueSharp)]
pub fn synagogue_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SynagogueSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,8v13h4v-7h4v7h4V8l-6-5L6,8z M13.5,10c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5S13.5,9.17,13.5,10z"/><path d="M3,5C1.9,5,1,5.9,1,7v1h4V7C5,5.9,4.1,5,3,5z"/><path d="M21,5c-1.1,0-2,0.9-2,2v1h4V7C23,5.9,22.1,5,21,5z"/>
        </SvgIcon>
    }
}
