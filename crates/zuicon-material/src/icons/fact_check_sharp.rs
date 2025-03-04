// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FactCheckSharp)]
pub fn fact_check_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FactCheckSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,3v18h20V3H2z M10,17H5v-2h5V17z M10,13H5v-2h5V13z M10,9H5V7h5V9z M14.82,15 L12,12.16l1.41-1.41l1.41,1.42L17.99,9l1.42,1.42L14.82,15z" fill-rule="evenodd"/>
        </SvgIcon>
    }
}
