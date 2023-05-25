// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraOutdoorRounded)]
pub fn camera_outdoor_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CameraOutdoorRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,13c0-0.55-0.45-1-1-1h-4c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1l1.27,0.67 C19.6,16.85,20,16.61,20,16.23v-2.46c0-0.38-0.4-0.62-0.73-0.44L18,14V13z M10.8,3.9l-6,4.5C4.3,8.78,4,9.37,4,10v9 c0,1.1,0.9,2,2,2h13c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6v-9l6-4.5l6,4.5l0,1h2v-1c0-0.63-0.3-1.22-0.8-1.6l-6-4.5 C12.49,3.37,11.51,3.37,10.8,3.9z"/>
        </SvgIcon>
    }
}
