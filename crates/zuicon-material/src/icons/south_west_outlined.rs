// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SouthWestOutlined)]
pub fn south_west_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SouthWestOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,19v-2H8.41L20,5.41L18.59,4L7,15.59V9H5v10H15z"/>
        </SvgIcon>
    }
}
