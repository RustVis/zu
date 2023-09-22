// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FileUploadTwoTone)]
pub fn file_upload_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FileUploadTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,10h4v6h6v-6h4l-7-7L5,10z M13,8v6h-2V8H9.83L12,5.83L14.17,8H13z"/>
        </SvgIcon>
    }
}
