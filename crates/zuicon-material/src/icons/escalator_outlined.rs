// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EscalatorOutlined)]
pub fn escalator_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EscalatorOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,5L19,5l0,14H5V5H19 M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2l0-14C21,3.9,20.1,3,19,3L19,3z M17,6h-3.3l-5,9H7c-0.83,0-1.5,0.67-1.5,1.5S6.17,18,7,18h3.3l5-9H17c0.83,0,1.5-0.67,1.5-1.5S17.83,6,17,6z"/>
        </SvgIcon>
    }
}
