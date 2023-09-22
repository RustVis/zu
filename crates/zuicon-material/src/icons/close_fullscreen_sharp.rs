// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CloseFullscreenSharp)]
pub fn close_fullscreen_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CloseFullscreenSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,3.41l-5.29,5.29L20,12h-8V4l3.29,3.29L20.59,2L22,3.41z M3.41,22l5.29-5.29L12,20v-8H4l3.29,3.29L2,20.59L3.41,22z"/>
        </SvgIcon>
    }
}
