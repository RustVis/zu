// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextFields)]
pub fn text_fields(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TextFields"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.5,4v3h5v12h3V7h5V4H2.5z M21.5,9h-9v3h3v7h3v-7h3V9z"/>
        </SvgIcon>
    }
}
