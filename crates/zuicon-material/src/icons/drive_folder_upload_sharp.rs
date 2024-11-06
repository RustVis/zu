// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DriveFolderUploadSharp)]
pub fn drive_folder_upload_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DriveFolderUploadSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,6H12l-2-2H2v16h20V6z M13,13v4h-2v-4H8l4.01-4L16,13H13z"/>
        </SvgIcon>
    }
}
