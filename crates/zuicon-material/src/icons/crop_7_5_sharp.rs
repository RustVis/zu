// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop75Sharp)]
pub fn crop_7_5_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop75Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,5H3v14h18V5z M19,17H5V7h14V17z"/>
        </SvgIcon>
    }
}
