// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HealthAndSafetySharp)]
pub fn health_and_safety_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HealthAndSafetySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.5,13H8v-3h2.5V7.5h3V10H16v3h-2.5v2.5h-3V13z M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2 z"/>
        </SvgIcon>
    }
}
