// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlipCameraIosOutlined)]
pub fn flip_camera_ios_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlipCameraIosOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,5h-3.17L15,3H9L7.17,5H4C2.9,5,2,5.9,2,7v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M20,19H4V7 h3.17h0.88l0.59-0.65L9.88,5h4.24l1.24,1.35L15.95,7h0.88H20V19z"/><path d="M12,17c-2.21,0-4-1.79-4-4h2l-2.5-2.5L5,13h2c0,2.76,2.24,5,5,5c0.86,0,1.65-0.24,2.36-0.62l-0.74-0.74 C13.13,16.87,12.58,17,12,17z"/><path d="M12,8c-0.86,0-1.65,0.24-2.36,0.62l0.74,0.73C10.87,9.13,11.42,9,12,9c2.21,0,4,1.79,4,4h-2l2.5,2.5L19,13h-2 C17,10.24,14.76,8,12,8z"/>
        </SvgIcon>
    }
}
