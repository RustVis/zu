// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoTransferRounded)]
pub fn no_transfer_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoTransferRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5.78,2.95C7.24,2.16,9.48,2,12,2c4.42,0,8,0.5,8,4v10c0,0.35-0.08,0.67-0.19,0.98L13.83,11H18V6H8.83L5.78,2.95z M20.49,21.9c-0.39,0.39-1.02,0.39-1.41,0l-1.01-1.01C17.89,20.96,17.7,21,17.5,21c-0.83,0-1.5-0.68-1.5-1.5V19H8v0.5 C8,20.33,7.33,21,6.5,21S5,20.33,5,19.5v-1.28C4.39,17.67,4,16.88,4,16V6.83l-1.9-1.9c-0.39-0.39-0.39-1.02,0-1.41 c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97C20.88,20.88,20.88,21.51,20.49,21.9z M9,15.5C9,14.67,8.33,14,7.5,14S6,14.67,6,15.5 C6,16.33,6.67,17,7.5,17S9,16.33,9,15.5z M8.17,11L6,8.83V11H8.17z"/>
        </SvgIcon>
    }
}
