// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop32Sharp)]
pub fn crop_3_2_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop32Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,6H3v12h18V6z M19,16H5V8h14V16z"/>
        </SvgIcon>
    }
}
