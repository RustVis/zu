// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThirtyFpsSelectOutlined)]
pub fn thirty_fps_select_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThirtyFpsSelectOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,4v2h5v2H5v2h4v2H4v2h5c1.1,0,2-0.9,2-2v-1.5C11,9.67,10.83,9,10,9c0.83,0,1-0.67,1-1.5V6c0-1.1-0.9-2-2-2H4z M18,4 c1.1,0,2,0.9,2,2v6c0,1.1-0.9,2-2,2h-3c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2H18z M18,6h-3v6h3V6z M5,22H3v-5h2V22z M9,22H7v-5h2V22 z M13,22h-2v-5h2V22z M21,22h-6v-5h6V22z"/>
        </SvgIcon>
    }
}
