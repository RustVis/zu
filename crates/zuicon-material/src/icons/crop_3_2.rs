// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop32)]
pub fn crop_3_2(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop32"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,6H5C3.9,6,3,6.9,3,8v8c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V8C21,6.9,20.1,6,19,6z M19,16H5V8h14V16z"/>
        </SvgIcon>
    }
}
