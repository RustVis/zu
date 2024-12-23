// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PauseTwoTone)]
pub fn pause_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PauseTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M6 5h4v14H6zm8 0h4v14h-4z"/>
        </SvgIcon>
    }
}
