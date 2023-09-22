// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GarageSharp)]
pub fn garage_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GarageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,2H2v20h20V2z M19,18.5h-2v-2H7v2H5v-7.31L6.89,5.5h10.21L19,11.19V18.5z"/>
        </SvgIcon>
    }
}
