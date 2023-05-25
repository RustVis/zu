// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SixtyFpsSharp)]
pub fn sixty_fps_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SixtyFpsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M19,8v8h-4V8H19 M22,5H12v14h10V5z M10,8V5H2v14h9v-9H5V8H10z M8,13v3H5v-3H8z"/>
        </SvgIcon>
    }
}
