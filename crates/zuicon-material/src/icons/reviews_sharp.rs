// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ReviewsSharp)]
pub fn reviews_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ReviewsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,2H2v20l4-4h16V2z M13.57,11.57L12,15l-1.57-3.43L7,10l3.43-1.57L12,5l1.57,3.43L17,10L13.57,11.57z"/>
        </SvgIcon>
    }
}
