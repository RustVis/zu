// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderDelete)]
pub fn folder_delete(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderDelete"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,8v10c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2L2.01,6C2.01,4.9,2.9,4,4,4h6l2,2h8C21.1,6,22,6.9,22,8z M16.5,10V9h-2v1H12 v1.5h1v4c0,0.83,0.67,1.5,1.5,1.5h2c0.83,0,1.5-0.67,1.5-1.5v-4h1V10H16.5z M16.5,15.5h-2v-4h2V15.5z"/>
        </SvgIcon>
    }
}
