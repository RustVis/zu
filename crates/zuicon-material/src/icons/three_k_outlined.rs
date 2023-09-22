// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThreeKOutlined)]
pub fn three_k_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThreeKOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M11,14v-4c0-0.55-0.45-1-1-1H6.5v1.5h3v1h-2v1h2v1h-3V15H10C10.55,15,11,14.55,11,14z"/>
        </SvgIcon>
    }
}
