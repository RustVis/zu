// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CountertopsOutlined)]
pub fn countertops_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CountertopsOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,10h-4V7c0-1.66-1.34-3-3-3c-1.66,0-3,1.34-3,3h2c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1v3H8c1.1,0,2-0.9,2-2V4H4v4 c0,1.1,0.9,2,2,2H2v2h2v8h16v-8h2V10z M6,6h2v2H6V6z M6,18v-6h5v6H6z M18,18h-5v-6h5V18z"/>
        </SvgIcon>
    }
}
