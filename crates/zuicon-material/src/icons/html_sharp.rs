// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HtmlSharp)]
pub fn html_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HtmlSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.5,9H5v6H3.5v-2.5h-2V15H0V9h1.5v2h2V9z M18.5,9H12v6h1.5v-4.5h1V14H16v-3.51h1V15h1.5V9z M11,9H6v1.5h1.75V15h1.5v-4.5 H11V9z M24,15v-1.5h-2.5V9H20v6H24z"/>
        </SvgIcon>
    }
}
