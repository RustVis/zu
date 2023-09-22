// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NorthSharp)]
pub fn north_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NorthSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,9l1.41,1.41L11,5.83V22H13V5.83l4.59,4.59L19,9l-7-7L5,9z"/>
        </SvgIcon>
    }
}
