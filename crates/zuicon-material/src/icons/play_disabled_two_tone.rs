// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlayDisabledTwoTone)]
pub fn play_disabled_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlayDisabledTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.81,2.81L1.39,4.22L8,10.83V19l4.99-3.18l6.78,6.78l1.41-1.41L2.81,2.81z M10,15.36v-2.53l1.55,1.55L10,15.36z M19,12 L8,5v0.17l8.45,8.45L19,12z"/>
        </SvgIcon>
    }
}
