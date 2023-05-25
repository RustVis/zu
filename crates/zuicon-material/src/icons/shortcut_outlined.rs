// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShortcutOutlined)]
pub fn shortcut_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShortcutOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M15,5l-1.41,1.41L15,7.83L17.17,10H8c-2.76,0-5,2.24-5,5v4h2v-4c0-1.65,1.35-3,3-3h9.17L15,14.17l-1.41,1.41L15,17l6-6 L15,5z"/>
        </SvgIcon>
    }
}
