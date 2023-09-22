// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderOff)]
pub fn folder_off(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderOff"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6h-8l-2-2H6.83l14.93,14.93C21.91,18.65,22,18.34,22,18V8C22,6.9,21.1,6,20,6z"/><path d="M2.1,2.1L0.69,3.51l1.56,1.56C2.1,5.35,2.01,5.66,2.01,6L2,18c0,1.1,0.9,2,2,2h13.17l3.31,3.31l1.41-1.41L2.1,2.1z"/>
        </SvgIcon>
    }
}
