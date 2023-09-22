// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartToySharp)]
pub fn smart_toy_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartToySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,9V5h-5V2H9v3H4v4H1v6h3v6h16v-6h3V9H20z M7.5,11.5C7.5,10.67,8.17,10,9,10s1.5,0.67,1.5,1.5S9.83,13,9,13 S7.5,12.33,7.5,11.5z M16,17H8v-2h8V17z M15,13c-0.83,0-1.5-0.67-1.5-1.5S14.17,10,15,10s1.5,0.67,1.5,1.5S15.83,13,15,13z"/>
        </SvgIcon>
    }
}
