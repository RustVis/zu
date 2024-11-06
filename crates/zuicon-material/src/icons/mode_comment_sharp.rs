// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ModeCommentSharp)]
pub fn mode_comment_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ModeCommentSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22 2H2v16h16l4 4z"/>
        </SvgIcon>
    }
}
