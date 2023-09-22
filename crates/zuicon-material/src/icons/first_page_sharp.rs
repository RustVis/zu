// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FirstPageSharp)]
pub fn first_page_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FirstPageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 0v24H0V0h24z" fill="none" opacity=".87"/><path d="M18.41 16.59L13.82 12l4.59-4.59L17 6l-6 6 6 6 1.41-1.41zM6 6h2v12H6V6z"/>
        </SvgIcon>
    }
}
