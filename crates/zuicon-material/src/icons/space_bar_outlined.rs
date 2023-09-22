// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpaceBarOutlined)]
pub fn space_bar_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpaceBarOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 9v4H6V9H4v6h16V9h-2z"/>
        </SvgIcon>
    }
}
