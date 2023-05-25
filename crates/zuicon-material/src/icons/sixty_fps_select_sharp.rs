// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SixtyFpsSelectSharp)]
pub fn sixty_fps_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SixtyFpsSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,6v6h-3V6H18z M20,4h-7v10h7V4z M11,6V4H4v10h7V8H6V6H11z M9,10v2H6v-2H9z M5,22H3v-5h2V22z M9,22H7v-5h2V22z M13,22 h-2v-5h2V22z M21,22h-6v-5h6V22z"/>
        </SvgIcon>
    }
}
