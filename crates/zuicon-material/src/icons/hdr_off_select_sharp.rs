// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrOffSelectSharp)]
pub fn hdr_off_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrOffSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,20v-4h-5v6h1.5v-2h1.1l0.9,2H18l-0.86-2L18,20z M16.5,18.5h-2v-1h2V18.5z M3.5,18h-2v-2H0v6h1.5v-2.5h2V22H5v-6H3.5 V18z M10,16H6.5v6H10c0.8,0,1.5-0.7,1.5-1.5v-3C11.5,16.7,10.8,16,10,16z M10,20.5H8v-3h2V20.5z M24,20h-2v2h-1.5v-2h-2v-1.5h2v-2 H22v2h2V20z M10.98,4.15L9.42,2.59c5.1-2.42,10.41,2.89,7.99,7.99l-1.56-1.56C16.66,6.06,13.94,3.34,10.98,4.15z M6.34,2.34 L4.93,3.76l1.66,1.66c-2.42,5.1,2.89,10.41,7.99,7.99l1.66,1.66l1.41-1.41L6.34,2.34z M8.15,6.98l4.87,4.87 C10.06,12.66,7.34,9.94,8.15,6.98z"/>
        </SvgIcon>
    }
}
