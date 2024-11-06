// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SuperscriptSharp)]
pub fn superscript_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SuperscriptSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,7v1h3v1h-4V6h3V5h-3V4h4v3H20z M5.88,20h2.66l3.4-5.42h0.12l3.4,5.42h2.66l-4.65-7.27L17.81,6h-2.68l-3.07,4.99h-0.12 L8.85,6H6.19l4.32,6.73L5.88,20z"/>
        </SvgIcon>
    }
}
