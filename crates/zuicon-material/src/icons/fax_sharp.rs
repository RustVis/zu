// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FaxSharp)]
pub fn fax_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FaxSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,9h-4V4H8v14.5V20h14V9z M10,6h6v3h-6V6z M14,17h-4v-5h4V17z M16,17c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C17,16.55,16.55,17,16,17z M16,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C17,13.55,16.55,14,16,14z M19,17 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C20,16.55,19.55,17,19,17z M19,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C20,13.55,19.55,14,19,14z"/>
        </SvgIcon>
    }
}
