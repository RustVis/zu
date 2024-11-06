// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FastForwardSharp)]
pub fn fast_forward_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FastForwardSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18l8.5-6L4,6V18z M13,6v12l8.5-6L13,6z"/>
        </SvgIcon>
    }
}
