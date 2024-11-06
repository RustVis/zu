// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SchemaOutlined)]
pub fn schema_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SchemaOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,9v2h-3V9H8.5V7H11V1H4v6h2.5v2H4v6h2.5v2H4v6h7v-6H8.5v-2H11v-2h3v2h7V9H14z M6,3h3v2H6V3z M9,21H6v-2h3V21z M9,13H6v-2 h3V13z M19,13h-3v-2h3V13z"/>
        </SvgIcon>
    }
}
