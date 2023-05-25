// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfySharp)]
pub fn view_comfy_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewComfySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v7h20V4H2z M10,20h12v-7H10V20z M2,20h6v-7H2V20z"/>
        </SvgIcon>
    }
}
