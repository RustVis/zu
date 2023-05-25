// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TimerOff)]
pub fn timer_off(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TimerOff"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,8v2.17l6.98,6.98C20.63,15.91,21,14.5,21,13c0-2.12-0.74-4.07-1.97-5.61l1.42-1.42c-0.43-0.51-0.9-0.99-1.41-1.41 l-1.42,1.42C16.07,4.74,14.12,4,12,4c-1.5,0-2.91,0.37-4.15,1.02L10.83,8H13z"/><path d="M2.81,2.81L1.39,4.22l3.4,3.4C3.67,9.12,3,10.98,3,13c0,4.97,4.02,9,9,9c2.02,0,3.88-0.67,5.38-1.79l2.4,2.4l1.41-1.41 L2.81,2.81z"/>
        </SvgIcon>
    }
}
