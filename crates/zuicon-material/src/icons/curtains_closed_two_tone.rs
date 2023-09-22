// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurtainsClosedTwoTone)]
pub fn curtains_closed_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurtainsClosedTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V3H4v16H2v2h20v-2H20z M9,19H6V5h3V19z M13,19h-2V5h2V19z M18,19h-3V5h3V19z"/>
        </SvgIcon>
    }
}
