// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DocumentScannerRounded)]
pub fn document_scanner_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DocumentScannerRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,6C2.45,6,2,5.55,2,5V2c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1S6.55,3,6,3H4v2C4,5.55,3.55,6,3,6z M17,2 c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V2c0-0.55-0.45-1-1-1h-3C17.45,1,17,1.45,17,2z M3,18c-0.55,0-1,0.45-1,1v3 c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H4v-2C4,18.45,3.55,18,3,18z M17,22c0,0.55,0.45,1,1,1h3 c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v2h-2C17.45,21,17,21.45,17,22z M19,18c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V6 c0-1.1,0.9-2,2-2h10c1.1,0,2,0.9,2,2V18z M9,9c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,8,9,8.45,9,9z M9,12c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,11,9,11.45,9,12z M9,15c0,0.55,0.45,1,1,1h4 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,14,9,14.45,9,15z"/>
        </SvgIcon>
    }
}
