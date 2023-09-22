// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop75)]
pub fn crop_7_5(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop75"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,5H5C3.9,5,3,5.9,3,7v10c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V7C21,5.9,20.1,5,19,5z M19,17H5V7h14V17z"/>
        </SvgIcon>
    }
}
