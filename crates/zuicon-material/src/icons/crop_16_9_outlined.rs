// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop169Outlined)]
pub fn crop_16_9_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop169Outlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,7H5C3.9,7,3,7.9,3,9v6c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V9C21,7.9,20.1,7,19,7z M19,15H5V9h14V15z"/>
        </SvgIcon>
    }
}
