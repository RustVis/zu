// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoCameraFrontSharp)]
pub fn video_camera_front_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoCameraFrontSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,10.48V4H2v16h16v-6.48l4,3.98v-11L18,10.48z M10,8c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S8.9,8,10,8z M14,16H6v-0.57 c0-0.81,0.48-1.53,1.22-1.85C8.07,13.21,9.01,13,10,13c0.99,0,1.93,0.21,2.78,0.58C13.52,13.9,14,14.62,14,15.43V16z"/>
        </SvgIcon>
    }
}
