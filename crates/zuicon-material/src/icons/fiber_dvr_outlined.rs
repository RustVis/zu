// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FiberDvrOutlined)]
pub fn fiber_dvr_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FiberDvrOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M11.87 12.43l-1-3.43h-1.5l1.75 6h1.5l1.75-6h-1.5zM21 11.5v-1c0-.85-.65-1.5-1.5-1.5H16v6h1.5v-2h1.15l.85 2H21l-.9-2.1c.5-.25.9-.8.9-1.4zm-1.5 0h-2v-1h2v1zM6.5 9H3v6h3.5c.85 0 1.5-.65 1.5-1.5v-3C8 9.65 7.35 9 6.5 9zm0 4.5h-2v-3h2v3z"/>
        </SvgIcon>
    }
}
