// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ModeNightOutlined)]
pub fn mode_night_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ModeNightOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M9.5,4c4.41,0,8,3.59,8,8s-3.59,8-8,8c-0.34,0-0.68-0.02-1.01-0.07c1.91-2.16,3.01-4.98,3.01-7.93s-1.1-5.77-3.01-7.93 C8.82,4.02,9.16,4,9.5,4 M9.5,2c-1.82,0-3.53,0.5-5,1.35c2.99,1.73,5,4.95,5,8.65s-2.01,6.92-5,8.65C5.97,21.5,7.68,22,9.5,22 c5.52,0,10-4.48,10-10S15.02,2,9.5,2z"/>
        </SvgIcon>
    }
}
