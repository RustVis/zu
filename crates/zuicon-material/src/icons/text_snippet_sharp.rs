// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextSnippetSharp)]
pub fn text_snippet_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TextSnippetSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,9l-6-6H3v18h18V9z M7,7h7v2H7V7z M17,17H7v-2h10V17z M17,13H7v-2h10V13z"/>
        </SvgIcon>
    }
}
