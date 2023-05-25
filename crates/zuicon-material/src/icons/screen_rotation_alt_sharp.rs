// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScreenRotationAltSharp)]
pub fn screen_rotation_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScreenRotationAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,7.59l6.41-6.41L20.24,11h-2.83L10.4,4L5.41,9H8v2H2V5h2V7.59z M20,19h2v-6h-6v2h2.59l-4.99,5l-7.01-7H3.76l9.83,9.83 L20,16.41V19z"/>
        </SvgIcon>
    }
}
