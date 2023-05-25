// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crop169Sharp)]
pub fn crop_16_9_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crop169Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,7H3v10h18V7z M19,15H5V9h14V15z"/>
        </SvgIcon>
    }
}
