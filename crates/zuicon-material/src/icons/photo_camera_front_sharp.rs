// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoCameraFrontSharp)]
pub fn photo_camera_front_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhotoCameraFrontSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.83,5L15,3H9L7.17,5H2v16h20V5H16.83z M12,9c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2s-2-0.9-2-2C10,9.9,10.9,9,12,9z M16,17H8 v-0.57c0-0.81,0.48-1.53,1.22-1.85C10.07,14.21,11.01,14,12,14s1.93,0.21,2.78,0.58C15.52,14.9,16,15.62,16,16.43V17z"/>
        </SvgIcon>
    }
}
