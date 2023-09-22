// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrAutoSelectSharp)]
pub fn hdr_auto_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrAutoSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M10,16H6.5v6H10c0.8,0,1.5-0.7,1.5-1.5v-3C11.5,16.7,10.8,16,10,16z M10,20.5H8v-3h2V20.5z"/><path d="M13,22h1.5v-2h1.1l0.9,2H18l-0.86-2H18v-4h-5V22z M14.5,17.5h2v1h-2V17.5z"/><path d="M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6s6-2.69,6-6S15.31,2,12,2z M14.04,11l-0.63-1.79h-2.83L9.96,11H8.74l2.63-7h1.25 l2.63,7H14.04z"/>
        </SvgIcon>
    }
}
