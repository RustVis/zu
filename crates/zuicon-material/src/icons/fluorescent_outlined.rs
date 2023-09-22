// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FluorescentOutlined)]
pub fn fluorescent_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FluorescentOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M5,15h14V9H5V15z M7,11h10v2H7V11z"/>
        </SvgIcon>
    }
}
