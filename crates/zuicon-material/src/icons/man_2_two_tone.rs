// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Man2TwoTone)]
pub fn man_2_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Man2TwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,7h-4C8.9,7,8,7.9,8,9v6h2.5v7h3v-7H16V9C16,7.9,15.1,7,14,7z"/>
        </SvgIcon>
    }
}
