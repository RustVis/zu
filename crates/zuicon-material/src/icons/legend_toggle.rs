// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LegendToggle)]
pub fn legend_toggle(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LegendToggle"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,15H4v-2h16V15z M20,17H4v2h16V17z M15,11l5-3.55L20,5l-5,3.55L10,5L4,8.66L4,11l5.92-3.61L15,11z"/>
        </SvgIcon>
    }
}
