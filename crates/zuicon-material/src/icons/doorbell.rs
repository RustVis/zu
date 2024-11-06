// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Doorbell)]
pub fn doorbell(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Doorbell"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,3L4,9v12h16V9L12,3z M12,17.5c-0.55,0-1-0.45-1-1h2C13,17.05,12.55,17.5,12,17.5z M16,16H8v-1h1v-2.34 c0-1.54,0.82-2.82,2.25-3.16V9.25c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75V9.5C14.19,9.84,15,11.12,15,12.66V15h1V16z"/>
        </SvgIcon>
    }
}
