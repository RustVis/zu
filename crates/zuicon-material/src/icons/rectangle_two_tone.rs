// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RectangleTwoTone)]
pub fn rectangle_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RectangleTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v16h20V4H2z M20,18H4V6h16V18z"/>
        </SvgIcon>
    }
}
