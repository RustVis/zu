// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewColumn)]
pub fn view_column(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewColumn"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.67,5v14H9.33V5H14.67z M15.67,19H21V5h-5.33V19z M8.33,19V5H3v14H8.33z"/>
        </SvgIcon>
    }
}
