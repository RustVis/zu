// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FileDownloadSharp)]
pub fn file_download_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FileDownloadSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9h-4V3H9v6H5l7,7L19,9z M5,18v2h14v-2H5z"/>
        </SvgIcon>
    }
}
