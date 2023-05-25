// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LineWeight)]
pub fn line_weight(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LineWeight"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,17h18v-2H3V17z M3,20h18v-1H3V20z M3,13h18v-3H3V13z M3,4v4h18V4H3z"/>
        </SvgIcon>
    }
}
