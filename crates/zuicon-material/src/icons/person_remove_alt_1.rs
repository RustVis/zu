// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PersonRemoveAlt1)]
pub fn person_remove_alt_1(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PersonRemoveAlt1"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z"/>
        </SvgIcon>
    }
}
