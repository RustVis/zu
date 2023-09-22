// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KingBedSharp)]
pub fn king_bed_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KingBedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,10V5H4v5H2v7h1.33L4,19h1l0.67-2h12.67L19,19h1l0.67-2H22v-7H20z M11,10H6V7h5V10z M18,10h-5V7h5V10z"/>
        </SvgIcon>
    }
}
