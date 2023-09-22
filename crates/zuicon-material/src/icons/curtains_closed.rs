// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurtainsClosed)]
pub fn curtains_closed(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurtainsClosed"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V3H4v16H2v2h20v-2H20z M11,5h2v14h-2V5z"/>
        </SvgIcon>
    }
}
