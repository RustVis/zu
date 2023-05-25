// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarginSharp)]
pub fn margin_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarginSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3v18h18V3H3z M9,13H7v-2h2V13z M9,9H7V7h2V9z M13,13h-2v-2h2V13z M13,9h-2V7h2V9z M17,13h-2v-2h2V13z M17,9h-2V7h2V9z"/>
        </SvgIcon>
    }
}
