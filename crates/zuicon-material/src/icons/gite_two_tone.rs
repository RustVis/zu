// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GiteTwoTone)]
pub fn gite_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GiteTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,12h10v5H4V12z M20,17h-4v-6.17l2-2v0l2,2V17z" opacity=".3"/><path d="M18,6H9V4H7v2H6l-4,4v9h20v-9L18,6z M4,12h10v5H4V12z M20,17h-4v-6.17l2-2v0l2,2V17z"/>
        </SvgIcon>
    }
}
