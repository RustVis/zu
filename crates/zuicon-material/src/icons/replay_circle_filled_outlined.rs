// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ReplayCircleFilledOutlined)]
pub fn replay_circle_filled_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ReplayCircleFilledOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,18.5c-3.31,0-6-2.69-6-6h2c0,2.21,1.79,4,4,4 s4-1.79,4-4c0-2.24-1.85-4.09-4.16-3.99l1.57,1.57L12,11.5l-4-4l4-4l1.41,1.41l-1.6,1.6C15.28,6.4,18,9.18,18,12.5 C18,15.81,15.31,18.5,12,18.5z"/>
        </SvgIcon>
    }
}
