// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FortTwoTone)]
pub fn fort_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FortTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,8.17L20.17,7h-4.34L17,8.17V12H7V8.17L8.17,7H3.83L5,8.17v7.66l-2,2V19h5v-1c0-2.21,1.79-4,4-4 s4,1.79,4,4v1h5v-1.17l-2-2V8.17z" opacity=".3"/><path d="M23,7V3h-2v2h-2V3h-2v2h-2V3h-2v4l2,2v1H9V9l2-2V3H9v2H7V3H5v2H3V3H1v4l2,2v6l-2,2v4h9v-3c0-1.1,0.9-2,2-2s2,0.9,2,2v3h9 v-4l-2-2V9L23,7z M21,19h-5v-1c0-2.21-1.79-4-4-4s-4,1.79-4,4v1H3v-1.17l2-2V8.17L3.83,7h4.34L7,8.17V12h10V8.17L15.83,7h4.34 L19,8.17v7.66l2,2V19z"/>
        </SvgIcon>
    }
}
