// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CoffeeOutlined)]
pub fn coffee_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CoffeeOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18.5,3H6C4.9,3,4,3.9,4,5v5.71c0,3.83,2.95,7.18,6.78,7.29c3.96,0.12,7.22-3.06,7.22-7v-1h0.5c1.93,0,3.5-1.57,3.5-3.5 S20.43,3,18.5,3z M16,5v3H6V5H16z M16,10v1c0,2.76-2.24,5-5,5s-5-2.24-5-5v-1 M18.5,8H18V5h0.5C19.33,5,20,5.67,20,6.5 S19.33,8,18.5,8z M4,19h16v2H4V19z"/>
        </SvgIcon>
    }
}
