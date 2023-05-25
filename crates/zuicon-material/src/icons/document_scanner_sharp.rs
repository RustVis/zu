// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DocumentScannerSharp)]
pub fn document_scanner_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DocumentScannerSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,3H4v3H2V1h5V3z M22,6V1h-5v2h3v3H22z M7,21H4v-3H2v5h5V21z M20,18v3h-3v2h5v-5H20z M19,4v16H5V4H19z M15,8H9v2h6V8z M15,11H9v2h6V11z M15,14H9v2h6V14z"/>
        </SvgIcon>
    }
}
