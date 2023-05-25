// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlinesRounded)]
pub fn airlines_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlinesRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.59,4h-5.01c-0.99,0-1.91,0.49-2.47,1.3L2,20h17l2.56-13.63C21.79,5.14,20.84,4,19.59,4z M14.5,14 c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5C17,12.88,15.88,14,14.5,14z"/>
        </SvgIcon>
    }
}
