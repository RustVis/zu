// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoStroller)]
pub fn no_stroller(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoStroller"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,18c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S4.9,18,6,18z M18.65,3c-1.66,0-2.54,1.27-3.18,2.03l-3.5,4.11L17,14.17v-7.9 C17.58,5.59,17.97,5,18.65,5C19.42,5,20,5.66,20,6.48V7h2V6.48C22,4.56,20.52,3,18.65,3z M10.67,10.67L2.81,2.81L1.39,4.22 l7.97,7.97L6.7,15.31c-0.55,0.65-0.09,1.65,0.76,1.65h6.66l1.17,1.17C14.54,18.42,14,19.14,14,20c0,1.1,0.9,2,2,2 c0.86,0,1.58-0.54,1.87-1.3l1.91,1.91l1.41-1.41l-4.8-4.8L10.67,10.67z M13.47,5.03c0.27-0.32,0.58-0.72,0.98-1.09 c-2.46-1.19-5.32-1.22-7.81-0.13l4.25,4.25L13.47,5.03z"/>
        </SvgIcon>
    }
}
