// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatItalicSharp)]
pub fn format_italic_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatItalicSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10 4v3h2.21l-3.42 8H6v3h8v-3h-2.21l3.42-8H18V4h-8z"/>
        </SvgIcon>
    }
}
