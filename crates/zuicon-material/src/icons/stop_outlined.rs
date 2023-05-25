// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StopOutlined)]
pub fn stop_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StopOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M16 8v8H8V8h8m2-2H6v12h12V6z"/>
        </SvgIcon>
    }
}
