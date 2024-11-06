// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(QrCode)]
pub fn qr_code(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("QrCode"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,11h8V3H3V11z M5,5h4v4H5V5z"/><path d="M3,21h8v-8H3V21z M5,15h4v4H5V15z"/><path d="M13,3v8h8V3H13z M19,9h-4V5h4V9z"/>
        </SvgIcon>
    }
}
