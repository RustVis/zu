// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowLeftSharp)]
pub fn arrow_left_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowLeftSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 0v24H0V0h24z" fill="none" opacity=".87"/><path d="M14 7l-5 5 5 5V7z"/>
        </SvgIcon>
    }
}
