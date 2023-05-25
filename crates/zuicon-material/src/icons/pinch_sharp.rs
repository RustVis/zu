// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PinchSharp)]
pub fn pinch_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PinchSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M23.18,15.4L22.1,23h-9L8,17.62l1.22-1.23L13,17.24V6.5C13,5.67,13.67,5,14.5,5S16,5.67,16,6.5v6h1.38L23.18,15.4z M6,2.5 V1h5v5H9.5V3.56L3.56,9.5H6V11H1V6h1.5v2.44L8.44,2.5H6z"/>
        </SvgIcon>
    }
}
