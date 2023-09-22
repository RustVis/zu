// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditOff)]
pub fn edit_off(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditOff"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0zm0 0h24v24H0z" fill="none"/><path d="M12.126 8.125l1.937-1.937 3.747 3.747-1.937 1.938zM20.71 5.63l-2.34-2.34a1 1 0 0 0-1.41 0l-1.83 1.83 3.75 3.75L20.71 7a1 1 0 0 0 0-1.37zM2 5l6.63 6.63L3 17.25V21h3.75l5.63-5.62L18 21l2-2L4 3 2 5z"/>
        </SvgIcon>
    }
}
