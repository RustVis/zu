// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraIndoorOutlined)]
pub fn camera_indoor_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CameraIndoorOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M14,13v-1c0-0.55-0.45-1-1-1H9c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1l2,1.06v-4.12L14,13z M12,5.5l6,4.5v9H6v-9L12,5.5 M12,3L4,9v12h16V9L12,3z"/>
        </SvgIcon>
    }
}
