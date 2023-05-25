// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SquareOutlined)]
pub fn square_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SquareOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3v18h18V3H3z M19,19H5V5h14V19z"/>
        </SvgIcon>
    }
}
