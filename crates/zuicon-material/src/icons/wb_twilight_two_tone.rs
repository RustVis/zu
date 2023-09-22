// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WbTwilightTwoTone)]
pub fn wb_twilight_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WbTwilightTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,16h14c0-3.87-3.13-7-7-7S5,12.13,5,16z"/>
        </SvgIcon>
    }
}
