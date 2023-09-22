// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WbShadeTwoTone)]
pub fn wb_shade_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WbShadeTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,12v2.5l5.5,5.5H22L14,12z M14,20h3l-3-3V20z M8,4l-6,6h2v10h8V10h2L8,4z M9,14H7v-4h2V14z"/>
        </SvgIcon>
    }
}
