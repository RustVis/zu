// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowCircleRightSharp)]
pub fn arrow_circle_right_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowCircleRightSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,12c0-5.52-4.48-10-10-10C6.48,2,2,6.48,2,12s4.48,10,10,10C17.52,22,22,17.52,22,12z M12,13H8v-2h4V8l4,4l-4,4V13z"/>
        </SvgIcon>
    }
}
