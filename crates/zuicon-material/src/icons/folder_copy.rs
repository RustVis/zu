// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderCopy)]
pub fn folder_copy(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderCopy"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,6H1v13c0,1.1,0.9,2,2,2h17v-2H3V6z"/><path d="M21,4h-7l-2-2H7C5.9,2,5.01,2.9,5.01,4L5,15c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V6C23,4.9,22.1,4,21,4z"/>
        </SvgIcon>
    }
}
