// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Filter2TwoTone)]
pub fn filter_2_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Filter2TwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M7 17h14V3H7v14zm4-6c0-1.11.9-2 2-2h2V7h-4V5h4c1.1 0 2 .89 2 2v2c0 1.11-.9 2-2 2h-2v2h4v2h-6v-4z" opacity=".3"/><path d="M17 13h-4v-2h2c1.1 0 2-.89 2-2V7c0-1.11-.9-2-2-2h-4v2h4v2h-2c-1.1 0-2 .89-2 2v4h6v-2zm4-12H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM1 21c0 1.1.9 2 2 2h16v-2H3V5H1v16z"/>
        </SvgIcon>
    }
}
