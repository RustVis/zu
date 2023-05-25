// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UploadFileTwoTone)]
pub fn upload_file_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UploadFileTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,4H6v16h12V9h-5V4z M16,15h-3v4h-2v-4H8l4.01-4L16,15z" opacity=".3"/><path d="M14,2H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2V8L14,2z M18,20H6V4h7v5h5V20z"/>
        </SvgIcon>
    }
}
