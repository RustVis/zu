// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThreeP)]
pub fn three_p(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThreeP"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2H4.01c-1.1,0-2,0.9-2,2L2,22l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M12,6c1.1,0,2,0.9,2,2s-0.9,2-2,2 s-2-0.9-2-2S10.9,6,12,6z M16,14H8v-0.57c0-0.81,0.48-1.53,1.22-1.85C10.07,11.21,11.01,11,12,11c0.99,0,1.93,0.21,2.78,0.58 C15.52,11.9,16,12.62,16,13.43V14z"/>
        </SvgIcon>
    }
}
