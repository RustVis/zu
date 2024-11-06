// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HomeRepairServiceSharp)]
pub fn home_repair_service_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HomeRepairServiceSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,8V4H7v4H2v6h4v-2h2v2h8v-2h2v2h4V8H17z M9,6h6v2H9V6z"/>
        </SvgIcon>
    }
}
