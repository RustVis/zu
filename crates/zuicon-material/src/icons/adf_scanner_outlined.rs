// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AdfScannerOutlined)]
pub fn adf_scanner_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AdfScannerOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,12h-1V4H6v8H5c-1.66,0-3,1.34-3,3v5h20v-5C22,13.34,20.66,12,19,12z M8,6h8v6H8V6z M20,18H4v-3c0-0.55,0.45-1,1-1h14 c0.55,0,1,0.45,1,1V18z"/>
        </SvgIcon>
    }
}
