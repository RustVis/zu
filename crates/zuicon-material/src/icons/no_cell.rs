// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoCell)]
pub fn no_cell(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoCell"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.83,6l-3.7-3.7C5.42,1.55,6.15,1,7,1l10,0.01c1.1,0,2,0.89,2,1.99v13.17l-2-2V6H8.83z M19.78,22.61l-0.91-0.91 C18.58,22.45,17.85,23,17,23H7c-1.1,0-2-0.9-2-2V7.83L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M15.17,18L7,9.83V18H15.17z"/>
        </SvgIcon>
    }
}
