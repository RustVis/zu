// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SaveAsSharp)]
pub fn save_as_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SaveAsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,12.4V7l-4-4H3v18h9.4L21,12.4z M15,15c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3S15,13.34,15,15z M6,6h9v4H6V6z M19.99,16.25l1.77,1.77L16.77,23H15v-1.77L19.99,16.25z M23.61,16.16l-1.2,1.2l-1.77-1.77l1.2-1.2L23.61,16.16z"/>
        </SvgIcon>
    }
}
