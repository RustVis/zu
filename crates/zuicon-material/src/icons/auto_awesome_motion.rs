// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoAwesomeMotion)]
pub fn auto_awesome_motion(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoAwesomeMotion"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M14 2H4c-1.11 0-2 .9-2 2v10h2V4h10V2zm4 4H8c-1.11 0-2 .9-2 2v10h2V8h10V6zm2 4h-8c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2z"/>
        </SvgIcon>
    }
}
