// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoCameraBackSharp)]
pub fn photo_camera_back_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhotoCameraBackSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.83,5L15,3H9L7.17,5H2v16h20V5H16.83z M6,17l3-4l2.25,3l3-4L18,17H6z"/>
        </SvgIcon>
    }
}
