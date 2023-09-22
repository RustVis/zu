// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeSharp)]
pub fn swipe_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.13,3.87C18.69,2.17,15.6,1,12,1S5.31,2.17,3.87,3.87L2,2v5h5L4.93,4.93c1-1.29,3.7-2.43,7.07-2.43 s6.07,1.14,7.07,2.43L17,7h5V2L20.13,3.87z"/><path d="M13,12.5v-6C13,5.67,12.33,5,11.5,5S10,5.67,10,6.5v10.74l-4.04-0.85l-1.21,1.23L10.13,23h8.97l1.09-7.64l-6.11-2.86H13z"/>
        </SvgIcon>
    }
}
