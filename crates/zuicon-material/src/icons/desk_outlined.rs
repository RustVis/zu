// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DeskOutlined)]
pub fn desk_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DeskOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,6v12h2V8h10v10h2v-2h4v2h2V6H2z M20,8v2h-4V8H20z M16,14v-2h4v2H16z"/>
        </SvgIcon>
    }
}
