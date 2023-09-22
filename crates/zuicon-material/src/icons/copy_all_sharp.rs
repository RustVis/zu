// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CopyAllSharp)]
pub fn copy_all_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CopyAllSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2H7v16h13V2z M18,16H9V4h9V16z M3,15v-2h2v2H3z M3,9.5h2v2H3V9.5z M10,20h2v2h-2V20z M3,18.5v-2h2v2H3z M5,22H3v-2h2V22 z M8.5,22h-2v-2h2V22z M15.5,22h-2v-2h2V22z M3,6h2v2H3V6z"/>
        </SvgIcon>
    }
}
