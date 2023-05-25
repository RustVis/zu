// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DragHandleTwoTone)]
pub fn drag_handle_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DragHandleTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M4 9h16v2H4zm0 4h16v2H4z"/>
        </SvgIcon>
    }
}
