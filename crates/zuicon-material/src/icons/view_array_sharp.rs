// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewArraySharp)]
pub fn view_array_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewArraySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,5h-3v14h3V5z M17,5H7v14h10V5z M6,5H3v14h3V5z"/>
        </SvgIcon>
    }
}
