// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrEnhancedSelectSharp)]
pub fn hdr_enhanced_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrEnhancedSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6s6-2.69,6-6S15.31,2,12,2z M15,9h-2v2h-2V9H9V7h2V5h2v2h2V9z"/><path d="M10,16H6.5v6H10c0.8,0,1.5-0.7,1.5-1.5v-3C11.5,16.7,10.8,16,10,16z M10,20.5H8v-3h2V20.5z"/><path d="M18,16h-5v6h1.5v-2h1.1l0.9,2H18l-0.86-2H18V16z M16.5,18.5h-2v-1h2V18.5z"/>
        </SvgIcon>
    }
}
