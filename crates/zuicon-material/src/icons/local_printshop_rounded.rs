// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LocalPrintshopRounded)]
pub fn local_printshop_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LocalPrintshopRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,7V4c0-0.55-0.45-1-1-1H7C6.45,3,6,3.45,6,4v3H18z"/><path d="M19,8H5c-1.66,0-3,1.34-3,3v5c0,0.55,0.45,1,1,1h3v2c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-2h3c0.55,0,1-0.45,1-1v-5 C22,9.34,20.66,8,19,8z M16,19H8v-4h8V19z M18,12.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S18.55,12.5,18,12.5z"/>
        </SvgIcon>
    }
}
