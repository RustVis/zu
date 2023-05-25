// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CommentsDisabledSharp)]
pub fn comments_disabled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CommentsDisabledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.83,14H18v-2h-3.17l-1-1H18V9h-6.17l-1-1H18V6H8.83l-4-4H22v17.17L16.83,14z M2.1,2.1L0.69,3.51L2,4.83V18h13.17 l5.31,5.31l1.41-1.41L2.1,2.1z M6,9h0.17l2,2H6V9z M6,14v-2h3.17l2,2H6z"/>
        </SvgIcon>
    }
}
