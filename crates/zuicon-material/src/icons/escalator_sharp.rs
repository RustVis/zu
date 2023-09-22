// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EscalatorSharp)]
pub fn escalator_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EscalatorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18L21,3z M18.5,9h-3.2l-5,9H5.5v-3h3.2l5-9h4.8V9z"/>
        </SvgIcon>
    }
}
