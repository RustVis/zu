// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebSharp)]
pub fn web_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,4H2v16h20V4z M4,9h10.5v3.5H4V9z M4,14.5h10.5V18L4,18V14.5z M20,18l-3.5,0V9H20V18z"/>
        </SvgIcon>
    }
}
