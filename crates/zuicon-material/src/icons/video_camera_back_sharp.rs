// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoCameraBackSharp)]
pub fn video_camera_back_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoCameraBackSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,10.48V4H2v16h16v-6.48l4,3.98v-11L18,10.48z M5,16l2.38-3.17L9,15l2.62-3.5L15,16H5z"/>
        </SvgIcon>
    }
}
