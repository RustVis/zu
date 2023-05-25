// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DriveFolderUploadRounded)]
pub fn drive_folder_upload_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DriveFolderUploadRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M13,13v3c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-3H9.21c-0.45,0-0.67-0.54-0.35-0.85l2.8-2.79 c0.2-0.2,0.51-0.19,0.71,0l2.79,2.79C15.46,12.46,15.24,13,14.8,13H13z"/>
        </SvgIcon>
    }
}
