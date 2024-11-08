// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InsertCommentSharp)]
pub fn insert_comment_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InsertCommentSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22 2H2v16h16l4 4V2zm-4 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z"/>
        </SvgIcon>
    }
}
