// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Inventory2Sharp)]
pub fn inventory_2_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Inventory2Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,2v6.7h1V22h18V8.7h1V2H2z M15,14H9v-2h6V14z M20,7H4V4h16V7z"/>
        </SvgIcon>
    }
}
