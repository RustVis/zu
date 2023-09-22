// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PriorityHighTwoTone)]
pub fn priority_high_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PriorityHighTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M10 3h4v12h-4z"/>
        </SvgIcon>
    }
}
