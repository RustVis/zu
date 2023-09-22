// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FastForwardOutlined)]
pub fn fast_forward_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FastForwardOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,9.86L18.03,12L15,14.14V9.86 M6,9.86L9.03,12L6,14.14V9.86 M13,6v12l8.5-6L13,6L13,6z M4,6v12l8.5-6L4,6L4,6z"/>
        </SvgIcon>
    }
}
