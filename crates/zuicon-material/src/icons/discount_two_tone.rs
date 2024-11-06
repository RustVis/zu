// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DiscountTwoTone)]
pub fn discount_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DiscountTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9.79L11.21,2H5v6.21L12.79,16L19,9.79z M7.25,5.5C6.56,5.5,6,4.94,6,4.25S6.56,3,7.25,3S8.5,3.56,8.5,4.25 S7.94,5.5,7.25,5.5z"/><path d="M12.79,21L3,11.21v2c0,0.53,0.21,1.04,0.59,1.41l7.79,7.79c0.78,0.78,2.05,0.78,2.83,0l6.21-6.21 c0.78-0.78,0.78-2.05,0-2.83L12.79,21z"/><path d="M11.38,17.41c0.39,0.39,0.9,0.59,1.41,0.59c0.51,0,1.02-0.2,1.41-0.59l6.21-6.21c0.78-0.78,0.78-2.05,0-2.83l-7.79-7.79 C12.25,0.21,11.74,0,11.21,0H5C3.9,0,3,0.9,3,2v6.21c0,0.53,0.21,1.04,0.59,1.41L11.38,17.41z M5,2h6.21L19,9.79L12.79,16L5,8.21 V2z"/>
        </SvgIcon>
    }
}
