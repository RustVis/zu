// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CallMade)]
pub fn call_made(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CallMade"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M9 5v2h6.59L4 18.59 5.41 20 17 8.41V15h2V5z"/>
        </SvgIcon>
    }
}
