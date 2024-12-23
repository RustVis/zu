// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Quickreply)]
pub fn quickreply(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Quickreply"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,4c0-1.1-0.9-2-2-2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h9v-8h7V4z"/>
        </SvgIcon>
    }
}
