// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Upload)]
pub fn upload(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Upload"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,20h14v-2H5V20z M5,10h4v6h6v-6h4l-7-7L5,10z"/>
        </SvgIcon>
    }
}
