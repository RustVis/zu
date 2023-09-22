// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderOpenSharp)]
pub fn folder_open_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderOpenSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 6H12l-2-2H2v16h20V6zm-2 12H4V8h16v10z"/>
        </SvgIcon>
    }
}
