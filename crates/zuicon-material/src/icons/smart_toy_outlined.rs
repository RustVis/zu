// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartToyOutlined)]
pub fn smart_toy_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartToyOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,9V7c0-1.1-0.9-2-2-2h-3c0-1.66-1.34-3-3-3S9,3.34,9,5H6C4.9,5,4,5.9,4,7v2c-1.66,0-3,1.34-3,3s1.34,3,3,3v4 c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4c1.66,0,3-1.34,3-3S21.66,9,20,9z M18,19L6,19V7h12V19z M9,13c-0.83,0-1.5-0.67-1.5-1.5 S8.17,10,9,10s1.5,0.67,1.5,1.5S9.83,13,9,13z M16.5,11.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5S14.17,10,15,10 S16.5,10.67,16.5,11.5z M8,15h8v2H8V15z"/>
        </SvgIcon>
    }
}
