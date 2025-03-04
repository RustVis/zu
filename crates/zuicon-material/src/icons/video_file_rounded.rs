// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoFileRounded)]
pub fn video_file_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoFileRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.17,2H6.01c-1.1,0-2,0.89-2,2L4,20c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8.83c0-0.53-0.21-1.04-0.59-1.41l-4.83-4.83 C14.21,2.21,13.7,2,13.17,2z M13,8V3.5L18.5,9H14C13.45,9,13,8.55,13,8z M14,14l1.27-0.67C15.6,13.15,16,13.39,16,13.77v2.46 c0,0.38-0.4,0.62-0.73,0.44L14,16v1c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1V14z"/>
        </SvgIcon>
    }
}
