// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AlignVerticalTopTwoTone)]
pub fn align_vertical_top_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AlignVerticalTopTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,2v2H2V2H22z M7,22h3V6H7V22z M14,16h3V6h-3V16z"/>
        </SvgIcon>
    }
}
