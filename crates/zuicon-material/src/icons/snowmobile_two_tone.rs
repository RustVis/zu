// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SnowmobileTwoTone)]
pub fn snowmobile_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SnowmobileTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,14h-6.7l-7.45-2.23l0.31-0.62L11.6,12l3.93-2.94c0,0,3.77,3.44,4.27,4.14C19.8,13.2,18.7,14,17,14z" opacity=".3"/><path d="M22,17c0,0.55-0.45,1-1,1h-0.17l-2.2-2.2C20.58,15.37,22,14.4,22,13c0-1-8-8-8-8h-3v2h2.25l0.8,0.72L11,10L2,9l-2,4 l4.54,1.36l-3.49,1.88C-0.77,17.22-0.07,20,2,20h6c2.21,0,4-1.79,4-4h4l2,2h-3v2h6c1.66,0,3-1.34,3-3H22z M8,18H2l5.25-2.83L10,16 C10,17.1,9.11,18,8,18z M17,14h-6.7l-7.45-2.23l0.31-0.62L11.6,12l3.93-2.94c0,0,3.77,3.44,4.27,4.14C19.8,13.2,18.7,14,17,14z"/>
        </SvgIcon>
    }
}
