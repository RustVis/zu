// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoCameraBackTwoTone)]
pub fn photo_camera_back_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhotoCameraBackTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.95,7l-1.83-2H9.88L8.05,7H4v12h16V7H15.95z M6,17l3-4l2.25,3l3-4L18,17H6z" opacity=".3"/><path d="M20,5h-3.17L15,3H9L7.17,5H4C2.9,5,2,5.9,2,7v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M20,19H4V7 h4.05l1.83-2h4.24l1.83,2H20V19z M11.25,16L9,13l-3,4h12l-3.75-5L11.25,16z"/>
        </SvgIcon>
    }
}
