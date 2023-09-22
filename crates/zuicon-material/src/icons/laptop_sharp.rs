// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LaptopSharp)]
pub fn laptop_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LaptopSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,18l2-2V4H2v12l2,2H0v2h24v-2H20z M4,6h16v10H4V6z"/>
        </SvgIcon>
    }
}
