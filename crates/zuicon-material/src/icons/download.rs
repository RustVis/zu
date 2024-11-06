// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Download)]
pub fn download(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Download"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,20h14v-2H5V20z M19,9h-4V3H9v6H5l7,7L19,9z"/>
        </SvgIcon>
    }
}
