// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WidthFullTwoTone)]
pub fn width_full_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WidthFullTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M5,18H4V6h1V18z M17,18H7V6h10V18z M20,18h-1V6h1V18z"/>
        </SvgIcon>
    }
}
