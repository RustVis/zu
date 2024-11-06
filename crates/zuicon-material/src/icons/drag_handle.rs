// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DragHandle)]
pub fn drag_handle(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DragHandle"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,9H4v2h16V9z M4,15h16v-2H4V15z"/>
        </SvgIcon>
    }
}
