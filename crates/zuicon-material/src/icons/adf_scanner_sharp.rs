// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AdfScannerSharp)]
pub fn adf_scanner_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AdfScannerSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,12h-4V4H6v8H2v8h20V12z M16,12H8V6h8V12z M18,17c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C19,16.55,18.55,17,18,17z"/>
        </SvgIcon>
    }
}
