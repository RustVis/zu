// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TwoKPlusOutlined)]
pub fn two_k_plus_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TwoKPlusOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,11.5h-1.5V10h-1v1.5H15v1h1.5 V14h1v-1.5H19V19H5V5h14V11.5z"/><path d="M10,13.5H7.5v-1H9c0.55,0,1-0.45,1-1V10c0-0.55-0.45-1-1-1H6v1.5h2.5v1H7c-0.55,0-1,0.45-1,1V15h4V13.5z"/>
        </SvgIcon>
    }
}
