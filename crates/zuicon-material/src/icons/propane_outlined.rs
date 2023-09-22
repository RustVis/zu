// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PropaneOutlined)]
pub fn propane_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PropaneOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,6h-1V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1H7c-3.31,0-6,2.69-6,6s2.69,6,6,6v3h2v-3h6v3h2v-3c3.31,0,6-2.69,6-6 S20.31,6,17,6z M10,5h4v1h-4V5z M17,16H7c-2.21,0-4-1.79-4-4s1.79-4,4-4h10c2.21,0,4,1.79,4,4S19.21,16,17,16z"/>
        </SvgIcon>
    }
}
