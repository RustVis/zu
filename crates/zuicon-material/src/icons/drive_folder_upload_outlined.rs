// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DriveFolderUploadOutlined)]
pub fn drive_folder_upload_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DriveFolderUploadOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6h-8l-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M20,18L4,18V6h5.17 l2,2H20V18z M9.41,14.42L11,12.84V17h2v-4.16l1.59,1.59L16,13.01L12.01,9L8,13.01L9.41,14.42z"/>
        </SvgIcon>
    }
}
