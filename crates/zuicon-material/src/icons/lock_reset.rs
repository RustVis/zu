// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LockReset)]
pub fn lock_reset(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LockReset"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,3c-4.97,0-9,4.03-9,9H1l4,4l4-4H6c0-3.86,3.14-7,7-7s7,3.14,7,7s-3.14,7-7,7c-1.9,0-3.62-0.76-4.88-1.99L6.7,18.42 C8.32,20.01,10.55,21,13,21c4.97,0,9-4.03,9-9S17.97,3,13,3z M15,11v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3 c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3C16,11.45,15.55,11,15,11z M14,11h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V11z"/>
        </SvgIcon>
    }
}
