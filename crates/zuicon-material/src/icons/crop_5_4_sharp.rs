// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop54Sharp)]
pub fn crop_5_4_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop54Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,4H3v16h18V4z M19,18H5V6h14V18z"/>
        </SvgIcon>
    }
}
