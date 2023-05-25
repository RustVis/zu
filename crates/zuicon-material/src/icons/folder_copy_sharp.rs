// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderCopySharp)]
pub fn folder_copy_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderCopySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M23,4h-9l-2-2H5.01L5,17h18V4z"/>
        </SvgIcon>
    }
}
