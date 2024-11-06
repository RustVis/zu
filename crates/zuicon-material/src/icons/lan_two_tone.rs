// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LanTwoTone)]
pub fn lan_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LanTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,7V4h4v3H10z M9,17v3H5v-3H9z M19,17v3h-4v-3H19z" opacity=".3"/><path d="M13,22h8v-7h-3v-4h-5V9h3V2H8v7h3v2H6v4H3v7h8v-7H8v-2h8v2h-3V22z M10,7V4h4v3H10z M9,17v3H5v-3H9z M19,17v3h-4v-3H19z"/>
        </SvgIcon>
    }
}
