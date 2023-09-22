// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoLuggageSharp)]
pub fn no_luggage_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoLuggageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.75,9v0.92l1.75,1.75V9H16v4.17l3,3V6h-4V2H9v4H8.83l3,3H12.75z M10.5,3.5h3V6h-3V3.5z M21.19,21.19L2.81,2.81L1.39,4.22 L5,7.83V21h2v1h2v-1h6v1h2v-1h1.17l1.61,1.61L21.19,21.19z M8,18v-7.17l1.5,1.5V18H8z M11.25,18v-3.92l1.5,1.5V18H11.25z"/>
        </SvgIcon>
    }
}
