// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ControlCameraTwoTone)]
pub fn control_camera_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ControlCameraTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M7.3 13.77L5.54 12l1.76-1.77-1.76-1.77L2 12l3.54 3.54zm8.24 4.69l-1.77-1.76L12 18.46l-1.77-1.76-1.77 1.76L12 22zm2.92-2.92L22 12l-3.54-3.54-1.76 1.77L18.46 12l-1.76 1.77zM12 5.54l1.77 1.76 1.77-1.76L12 2 8.46 5.54l1.77 1.76z"/>
        </SvgIcon>
    }
}
