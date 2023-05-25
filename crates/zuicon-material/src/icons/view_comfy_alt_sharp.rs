// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfyAltSharp)]
pub fn view_comfy_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewComfyAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v16h20V4H2z M11,17H7v-4h4V17z M11,11H7V7h4V11z M17,17h-4v-4h4V17z M17,11h-4V7h4V11z"/>
        </SvgIcon>
    }
}
