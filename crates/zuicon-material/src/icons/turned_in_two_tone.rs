// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnedInTwoTone)]
pub fn turned_in_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnedInTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2zm0 14.97l-4.21-1.81-.79-.34-.79.34L7 17.97V5h10v12.97z"/><path d="M7 17.97l4.21-1.81.79-.34.79.34L17 17.97V5H7z" opacity=".3"/>
        </SvgIcon>
    }
}
