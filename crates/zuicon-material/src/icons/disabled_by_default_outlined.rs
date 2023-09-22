// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DisabledByDefaultOutlined)]
pub fn disabled_by_default_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DisabledByDefaultOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,19H5V5h14V19z M3,3v18h18V3H3z M17,15.59L15.59,17L12,13.41L8.41,17L7,15.59L10.59,12L7,8.41L8.41,7L12,10.59L15.59,7 L17,8.41L13.41,12L17,15.59z"/>
        </SvgIcon>
    }
}
