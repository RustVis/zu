// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoCameraBackRounded)]
pub fn video_camera_back_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoCameraBackRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,10.48V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4.48l3.15,3.13 C21.46,16.97,22,16.74,22,16.3V7.7c0-0.44-0.54-0.67-0.85-0.35L18,10.48z M5.6,15.2l1.38-1.83c0.2-0.27,0.6-0.27,0.8,0L9,15 l2.23-2.97c0.2-0.27,0.6-0.27,0.8,0l2.38,3.17c0.25,0.33,0.01,0.8-0.4,0.8H6C5.59,16,5.35,15.53,5.6,15.2z"/>
        </SvgIcon>
    }
}
