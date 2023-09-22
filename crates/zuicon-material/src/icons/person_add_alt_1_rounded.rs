// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PersonAddAlt1Rounded)]
pub fn person_add_alt_1_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PersonAddAlt1Rounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,14c-2.67,0-8,1.34-8,4v1c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-1C17,15.34,11.67,14,9,14z"/>
        </SvgIcon>
    }
}
