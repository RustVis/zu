// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartDisplayTwoTone)]
pub fn smart_display_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartDisplayTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18.01h16V5.99H4V18.01z M9.5,7.5l7,4.5l-7,4.5V7.5z" opacity=".3"/><path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M20,18.01H4V5.99h16V18.01z"/>
        </SvgIcon>
    }
}
