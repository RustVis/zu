// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraswitchOutlined)]
pub fn cameraswitch_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CameraswitchOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,7h-1l-1-1h-4L9,7H8C6.9,7,6,7.9,6,9v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V9C18,7.9,17.1,7,16,7z M16,15H8V9h1.83l1-1 h2.34l1,1H16V15z"/><path d="M8.57,0.52L13.05,5l1.41-1.41l-1.54-1.54C17.7,2.46,21.53,6.24,22,11h2C23.36,3.3,15.79-1.67,8.57,0.52z"/><path d="M9.54,20.41l1.54,1.54C6.3,21.54,2.47,17.76,2,13H0c0.64,7.7,8.21,12.67,15.43,10.48L10.95,19L9.54,20.41z"/>
        </SvgIcon>
    }
}
