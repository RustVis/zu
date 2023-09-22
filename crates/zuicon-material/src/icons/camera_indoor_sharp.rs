// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraIndoorSharp)]
pub fn camera_indoor_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CameraIndoorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,3L4,9v12h16V9L12,3z M16,16.06L14,15v2H8v-6h6v2l2-1.06V16.06z"/>
        </SvgIcon>
    }
}
