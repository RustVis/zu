// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GradingOutlined)]
pub fn grading_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GradingOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,7h16v2H4V7z M4,13h16v-2H4V13z M4,17h7v-2H4V17z M4,21h7v-2H4V21z M15.41,18.17L14,16.75l-1.41,1.41L15.41,21L20,16.42 L18.58,15L15.41,18.17z M4,3v2h16V3H4z"/>
        </SvgIcon>
    }
}
