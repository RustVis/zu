// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChairSharp)]
pub fn chair_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChairSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M23,9h-4v6H5V9H1v10h3v1c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-1h12v1c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-1h3V9z"/>
        </SvgIcon>
    }
}
