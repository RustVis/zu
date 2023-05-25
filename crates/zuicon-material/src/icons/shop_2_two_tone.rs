// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Shop2TwoTone)]
pub fn shop_2_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Shop2TwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,16h14V7H7V16z M12,8l5.5,3.5L12,15V8z" enable-background="new" opacity=".3"/><path d="M3,9H1v11c0,1.11,0.89,2,2,2h16v-2H3V9z"/><path d="M18,5V3c0-1.11-0.89-2-2-2h-4c-1.11,0-2,0.89-2,2v2H5v11c0,1.11,0.89,2,2,2h14c1.11,0,2-0.89,2-2V5H18z M12,3h4v2h-4V3z M21,16H7V7h14V16z"/>
        </SvgIcon>
    }
}
