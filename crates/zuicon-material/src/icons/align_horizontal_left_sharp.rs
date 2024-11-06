// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AlignHorizontalLeftSharp)]
pub fn align_horizontal_left_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AlignHorizontalLeftSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,22H2V2h2V22z M22,7H6v3h16V7z M16,14H6v3h10V14z"/>
        </SvgIcon>
    }
}
