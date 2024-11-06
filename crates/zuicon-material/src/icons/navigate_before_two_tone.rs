// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NavigateBeforeTwoTone)]
pub fn navigate_before_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NavigateBeforeTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M14.2 6l-6 6 6 6 1.41-1.41L11.03 12l4.58-4.59z"/>
        </SvgIcon>
    }
}
