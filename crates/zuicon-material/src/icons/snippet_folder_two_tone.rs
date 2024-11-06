// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SnippetFolderTwoTone)]
pub fn snippet_folder_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SnippetFolderTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.17,6H4v12l16,0V8h-8.83L9.17,6z" opacity=".3"/><path d="M20,6h-8l-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M20,18L4,18V6h5.17 l2,2H20V18z M17.5,12.12v3.38l-3,0v-5h1.38L17.5,12.12z M16.5,9H13v8l6,0v-5.5L16.5,9L16.5,9z"/>
        </SvgIcon>
    }
}
