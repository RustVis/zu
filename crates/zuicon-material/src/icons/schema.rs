// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Schema)]
pub fn schema(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Schema"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,9v2h-3V9H8.5V7H11V1H4v6h2.5v2H4v6h2.5v2H4v6h7v-6H8.5v-2H11v-2h3v2h7V9H14z"/>
        </SvgIcon>
    }
}
