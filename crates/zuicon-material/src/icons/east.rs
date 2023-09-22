// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(East)]
pub fn east(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("East"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,5l-1.41,1.41L18.17,11H2V13h16.17l-4.59,4.59L15,19l7-7L15,5z"/>
        </SvgIcon>
    }
}
