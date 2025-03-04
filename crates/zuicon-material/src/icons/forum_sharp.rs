// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForumSharp)]
pub fn forum_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForumSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 6h-3v9H6v3h12l4 4V6zm-5 7V2H2v15l4-4h11z"/>
        </SvgIcon>
    }
}
