// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShortText)]
pub fn short_text(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShortText"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,9h16v2H4V9z M4,13h10v2H4V13z"/>
        </SvgIcon>
    }
}
