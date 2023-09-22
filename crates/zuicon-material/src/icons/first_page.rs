// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FirstPage)]
pub fn first_page(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FirstPage"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.41 16.59L13.82 12l4.59-4.59L17 6l-6 6 6 6zM6 6h2v12H6z"/><path d="M24 24H0V0h24v24z" fill="none"/>
        </SvgIcon>
    }
}
