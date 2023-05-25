// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextDecreaseTwoTone)]
pub fn text_decrease_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TextDecreaseTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0.99,19h2.42l1.27-3.58h5.65L11.59,19h2.42L8.75,5h-2.5L0.99,19z M5.41,13.39L7.44,7.6h0.12l2.03,5.79H5.41z M23,11v2h-8 v-2H23z"/>
        </SvgIcon>
    }
}
