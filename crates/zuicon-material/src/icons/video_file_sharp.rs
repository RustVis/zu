// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoFileSharp)]
pub fn video_file_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoFileSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H4v20h16V8L14,2z M13,9V3.5L18.5,9H13z M14,14l2-1.06v4.12L14,16v2H8v-6h6V14z"/>
        </SvgIcon>
    }
}
