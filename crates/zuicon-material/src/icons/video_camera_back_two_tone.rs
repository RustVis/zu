// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoCameraBackTwoTone)]
pub fn video_camera_back_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoCameraBackTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18h12V6H4V18z M7.38,12.83L9,15l2.62-3.5L15,16H5L7.38,12.83z" opacity=".3"/><path d="M18,10.48V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4.48l4,3.98v-11L18,10.48z M16,18 H4V6h12V18z"/>
        </SvgIcon>
    }
}
