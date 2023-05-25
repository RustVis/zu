// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrEnhancedSelectTwoTone)]
pub fn hdr_enhanced_select_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrEnhancedSelectTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4C9.79,4,8,5.79,8,8s1.79,4,4,4s4-1.79,4-4S14.21,4,12,4z M15,9h-2v2h-2V9H9V7h2V5h2v2h2V9z" opacity=".3"/><path d="M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6s6-2.69,6-6S15.31,2,12,2z M12,12c-2.21,0-4-1.79-4-4s1.79-4,4-4s4,1.79,4,4 S14.21,12,12,12z"/><path d="M16.5,16H13v6h1.5v-2h1.1l0.9,2H18l-0.9-2.1c0.5-0.3,0.9-0.8,0.9-1.4v-1C18,16.7,17.3,16,16.5,16z M16.5,18.5h-2v-1h2 V18.5z"/><path d="M10,16H6.5v6H10c0.8,0,1.5-0.7,1.5-1.5v-3C11.5,16.7,10.8,16,10,16z M10,20.5H8v-3h2V20.5z"/>
        </SvgIcon>
    }
}
