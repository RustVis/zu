// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DiscountSharp)]
pub fn discount_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DiscountSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,9.04l9.79,9.79l9.04-9.04L12.04,0H3V9.04z M7.25,3C7.94,3,8.5,3.56,8.5,4.25S7.94,5.5,7.25,5.5S6,4.94,6,4.25 S6.56,3,7.25,3z"/>
        </SvgIcon>
    }
}
