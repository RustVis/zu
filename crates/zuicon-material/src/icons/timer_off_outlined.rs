// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TimerOffOutlined)]
pub fn timer_off_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TimerOffOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,6c3.87,0,7,3.13,7,7c0,0.94-0.19,1.83-0.52,2.65l1.5,1.5C20.63,15.91,21,14.5,21,13c0-2.12-0.74-4.07-1.97-5.61 l1.42-1.42c-0.43-0.51-0.9-0.99-1.41-1.41l-1.42,1.42C16.07,4.74,14.12,4,12,4c-1.5,0-2.91,0.37-4.15,1.02l1.5,1.5 C10.17,6.19,11.06,6,12,6z"/><path d="M2.81,2.81L1.39,4.22l3.4,3.4C3.67,9.12,3,10.98,3,13c0,4.97,4.02,9,9,9c2.02,0,3.88-0.67,5.38-1.79l2.4,2.4l1.41-1.41 L2.81,2.81z M12,20c-3.87,0-7-3.13-7-7c0-1.47,0.45-2.83,1.22-3.95l9.73,9.73C14.83,19.55,13.47,20,12,20z"/>
        </SvgIcon>
    }
}
