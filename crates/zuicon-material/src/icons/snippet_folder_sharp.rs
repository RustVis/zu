// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SnippetFolderSharp)]
pub fn snippet_folder_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SnippetFolderSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,6l-2-2H2v16h20V6H12z M19,17l-6,0V9h3.5l2.5,2.5V17z M15.88,10.5l1.62,1.62v3.38l-3,0v-5H15.88z"/>
        </SvgIcon>
    }
}
