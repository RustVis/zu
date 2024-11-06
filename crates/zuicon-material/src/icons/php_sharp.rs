// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhpSharp)]
pub fn php_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,9h1.5v6H13v-2.5h-2V15H9.5V9H11v2h2V9z M8,9v4H4.5v2H3V9H8z M6.5,10.5h-2v1h2V10.5z M21.5,9v4H18v2h-1.5V9H21.5z M20,10.5h-2v1h2V10.5z"/>
        </SvgIcon>
    }
}
