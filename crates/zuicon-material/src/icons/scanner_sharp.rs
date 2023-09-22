// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScannerSharp)]
pub fn scanner_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScannerSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M4.2 5l-.7 1.9L17.6 12H3v8h18v-8.86L4.2 5zM7 17H5v-2h2v2zm12 0H9v-2h10v2z"/>
        </SvgIcon>
    }
}
