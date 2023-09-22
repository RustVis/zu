// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowCircleLeftTwoTone)]
pub fn arrow_circle_left_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowCircleLeftTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,12c0,4.41-3.59,8-8,8s-8-3.59-8-8s3.59-8,8-8S20,7.59,20,12 M12,13l4,0v-2l-4,0V8l-4,4l4,4V13z" opacity=".3"/><path d="M20,12c0,4.41-3.59,8-8,8s-8-3.59-8-8s3.59-8,8-8S20,7.59,20,12 M22,12c0-5.52-4.48-10-10-10C6.48,2,2,6.48,2,12 c0,5.52,4.48,10,10,10C17.52,22,22,17.52,22,12L22,12z M12,13l4,0v-2l-4,0V8l-4,4l4,4V13z"/>
        </SvgIcon>
    }
}
