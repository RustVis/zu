// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpaceDashboardSharp)]
pub fn space_dashboard_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpaceDashboardSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,21H3V3h8V21z M13,21h8v-9h-8V21z M21,10V3h-8v7H21z"/>
        </SvgIcon>
    }
}
