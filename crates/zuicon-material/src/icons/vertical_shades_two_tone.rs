// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VerticalShadesTwoTone)]
pub fn vertical_shades_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VerticalShadesTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V3H4v16H2v2h20v-2H20z M8,19H6V5h2V19z M14,19h-4V5h4V19z M18,19h-2V5h2V19z"/>
        </SvgIcon>
    }
}
