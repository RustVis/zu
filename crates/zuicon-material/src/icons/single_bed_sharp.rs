// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SingleBedSharp)]
pub fn single_bed_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SingleBedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,10V5H6v5H4v7h1.33L6,19h1l0.67-2h8.67L17,19h1l0.67-2H20v-7H18z M11,10H8V7h3V10z M16,10h-3V7h3V10z"/>
        </SvgIcon>
    }
}
