// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BlindsClosedTwoTone)]
pub fn blinds_closed_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BlindsClosedTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V3H4v16H2v2h11.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M14,19H6v-2h8V19z M14,15H6v-2h8V15z M14,11H6V9h8V11z M14,7H6V5h8V7z M18,19h-2v-2h2V19z M18,15h-2v-2h2V15z M18,11h-2V9h2V11z M18,7h-2V5h2V7z"/>
        </SvgIcon>
    }
}
