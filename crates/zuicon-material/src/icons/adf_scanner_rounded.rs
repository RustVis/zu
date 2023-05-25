// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AdfScannerRounded)]
pub fn adf_scanner_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AdfScannerRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,12h-1V6c0-1.1-0.9-2-2-2H8C6.9,4,6,4.9,6,6v6H5c-1.66,0-3,1.34-3,3v3c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-3 C22,13.34,20.66,12,19,12z M16,12H8V6h8V12z M18,17c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C19,16.55,18.55,17,18,17z"/>
        </SvgIcon>
    }
}
