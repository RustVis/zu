// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrOnSelectSharp)]
pub fn hdr_on_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrOnSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,19.9V16h-5v6h1.5v-2h1.1l0.9,2H18l-0.9-2.1H18z M16.5,18.5h-2v-1h2V18.5z M3.5,18h-2v-2H0v6h1.5v-2.5h2V22H5v-6H3.5 V18z M10,16H6.5v6H10c0.8,0,1.5-0.7,1.5-1.5v-3C11.5,16.7,10.8,16,10,16z M10,20.5H8v-3h2V20.5z M24,20h-2v2h-1.5v-2h-2v-1.5h2v-2 H22v2h2V20z M12,4c2.21,0,4,1.79,4,4s-1.79,4-4,4s-4-1.79-4-4S9.79,4,12,4 M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6s6-2.69,6-6 S15.31,2,12,2z"/>
        </SvgIcon>
    }
}
