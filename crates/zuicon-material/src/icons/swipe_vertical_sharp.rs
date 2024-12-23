// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeVerticalSharp)]
pub fn swipe_vertical_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeVerticalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1,3.5h2.02C1.13,5.82,0,8.78,0,12s1.13,6.18,3.02,8.5H1V22h5v-5H4.5v2.91c-1.86-2.11-3-4.88-3-7.91s1.14-5.79,3-7.91V7H6 V2H1V3.5z M21.71,11.18l2.09,7.39l-8.23,3.65l-6.84-2.85l0.61-1.62l3.8-0.75L8.79,7.17c-0.34-0.76,0-1.64,0.76-1.98 c0.76-0.34,1.64,0,1.98,0.76l2.43,5.49l1.26-0.56L21.71,11.18z"/>
        </SvgIcon>
    }
}
