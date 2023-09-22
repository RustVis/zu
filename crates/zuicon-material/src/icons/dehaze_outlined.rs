// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DehazeOutlined)]
pub fn dehaze_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DehazeOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M2 16v2h20v-2H2zm0-5v2h20v-2H2zm0-5v2h20V6H2z"/>
        </SvgIcon>
    }
}
