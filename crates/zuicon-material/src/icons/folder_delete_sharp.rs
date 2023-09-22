// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderDeleteSharp)]
pub fn folder_delete_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderDeleteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,6v14H2V4h8l2,2H22z M16.5,10V9h-2v1H12v1.5h1V17h5v-5.5h1V10H16.5z M16.5,15.5h-2v-4h2V15.5z"/>
        </SvgIcon>
    }
}
